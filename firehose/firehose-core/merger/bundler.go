// Copyright 2019 dfuse Platform Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package merger

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"io"
	"sync"
	"time"

	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/bstream/forkable"
	"github.com/streamingfast/firehose-core/merger/metrics"
	"github.com/streamingfast/logging"
	"go.uber.org/zap"
)

var ErrStopBlockReached = errors.New("stop block reached")
var ErrFirstBlockAfterInitialStreamableBlock = errors.New("received first block after inital streamable block")

type Bundler struct {
	sync.Mutex

	io IOInterface

	baseBlockNum uint64

	bundleSize                 uint64
	bundleError                chan error
	inProcess                  sync.Mutex
	stopBlock                  uint64
	enforceNextBlockOnBoundary bool
	firstStreamableBlock       uint64

	seenBlockFiles     map[string]*bstream.OneBlockFile
	irreversibleBlocks []*bstream.OneBlockFile
	forkable           *forkable.Forkable

	logger *zap.Logger
}

var logger, _ = logging.PackageLogger("merger", "github.com/streamingfast/firehose-core/merger/bundler")

func NewBundler(startBlock, stopBlock, firstStreamableBlock, bundleSize uint64, io IOInterface) *Bundler {
	b := &Bundler{
		bundleSize:           bundleSize,
		io:                   io,
		bundleError:          make(chan error, 1),
		firstStreamableBlock: firstStreamableBlock,
		stopBlock:            stopBlock,
		seenBlockFiles:       make(map[string]*bstream.OneBlockFile),
		logger:               logger,
	}
	b.Reset(toBaseNum(startBlock, bundleSize), nil)
	return b
}

// BaseBlockNum can be called from a different thread
func (b *Bundler) BaseBlockNum() uint64 {
	b.inProcess.Lock()
	defer b.inProcess.Unlock()
	// while inProcess is locked, all blocks below b.baseBlockNum are actually merged
	return b.baseBlockNum
}

func (b *Bundler) HandleBlockFile(obf *bstream.OneBlockFile) error {
	b.seenBlockFiles[obf.CanonicalName] = obf
	return b.forkable.ProcessBlock(obf.ToBstreamBlock(), obf) // forkable will call our own b.ProcessBlock() on irreversible blocks only
}

func (b *Bundler) forkedBlocksInCurrentBundle() (out []*bstream.OneBlockFile) {
	highBoundary := b.baseBlockNum + b.bundleSize

	// remove irreversible blocks from map (they will be merged and deleted soon)
	for _, block := range b.irreversibleBlocks {
		delete(b.seenBlockFiles, block.CanonicalName)
	}

	// identify and then delete remaining blocks from map, return them as forks
	for name, block := range b.seenBlockFiles {
		if block.Num < b.baseBlockNum {
			delete(b.seenBlockFiles, name) // too old, just cleaning up the map of lingering old blocks
		}
		if block.Num < highBoundary {
			out = append(out, block)
			delete(b.seenBlockFiles, name)
		}
	}
	return
}

func (b *Bundler) Reset(nextBase uint64, lib bstream.BlockRef) {
	options := []forkable.Option{
		forkable.WithFilters(bstream.StepIrreversible),
		forkable.HoldBlocksUntilLIB(),
		forkable.WithWarnOnUnlinkableBlocks(100), // don't warn too soon, sometimes oneBlockFiles are uploaded out of order from mindreader (on remote I/O)
	}
	if lib != nil {
		options = append(options, forkable.WithInclusiveLIB(lib))
		b.enforceNextBlockOnBoundary = false // we don't need to check first block because we know it will be linked to lib
	} else {
		b.enforceNextBlockOnBoundary = true
	}
	b.forkable = forkable.New(b, options...)

	b.Lock()
	b.baseBlockNum = nextBase
	b.irreversibleBlocks = nil
	b.Unlock()
}

func readBlockTime(data []byte) (time.Time, error) {
	reader := bytes.NewReader(data)
	blockReader, err := bstream.NewDBinBlockReader(reader)
	if err != nil {
		return time.Time{}, fmt.Errorf("unable to create block reader: %w", err)
	}
	blk, err := blockReader.Read()
	if err != nil && err != io.EOF {
		return time.Time{}, fmt.Errorf("block reader failed: %w", err)
	}
	return blk.Time(), nil
}

func (b *Bundler) ProcessBlock(_ *pbbstream.Block, obj interface{}) error {
	obf := obj.(bstream.ObjectWrapper).WrappedObject().(*bstream.OneBlockFile)
	if obf.Num < b.baseBlockNum {
		// we may be receiving an inclusive LIB just before our bundle, ignore it
		return nil
	}

	if b.enforceNextBlockOnBoundary {
		if obf.Num != b.baseBlockNum && obf.Num != b.firstStreamableBlock {
			//{"severity":"ERROR","timestamp":"2023-11-07T12:28:34.735713163-05:00","logger":"merger","message":"expecting to start at block `base_block_num` but got block `block_num` (and we have no previous blockID to align with..). First streamable block is configured to be: `first_streamable_block`",
			//"base_block_num":22207900,
			//"block_num":22208900,
			//"first_streamable_block":22207900,
			//"logging.googleapis.com/labels":{},"serviceContext":{"service":"unknown"}}
			b.logger.Error(
				"expecting to start at block `base_block_num` but got block `block_num` (and we have no previous blockID to align with..). First streamable block is configured to be: `first_streamable_block`",
				zap.Uint64("base_block_num", b.baseBlockNum),
				zap.Uint64("block_num", obf.Num),
				zap.Uint64("first_streamable_block", b.firstStreamableBlock),
			)
			return ErrFirstBlockAfterInitialStreamableBlock
		}
		b.enforceNextBlockOnBoundary = false
	}

	if obf.Num < b.baseBlockNum+b.bundleSize {
		b.Lock()
		metrics.AppReadiness.SetReady()
		b.irreversibleBlocks = append(b.irreversibleBlocks, obf)
		metrics.HeadBlockNumber.SetUint64(obf.Num)
		go func() {
			// this pre-downloads the data
			data, err := obf.Data(context.Background(), b.io.DownloadOneBlockFile)
			if err != nil {
				return
			}
			// now that we have the data, might as well read the block time for metrics
			if time, err := readBlockTime(data); err == nil {
				metrics.HeadBlockTimeDrift.SetBlockTime(time)
			}
		}()
		b.Unlock()
		return nil
	}

	select {
	case err := <-b.bundleError:
		return err
	default:
	}

	forkedBlocks := b.forkedBlocksInCurrentBundle()
	blocksToBundle := b.irreversibleBlocks
	baseBlockNum := b.baseBlockNum
	b.inProcess.Lock()
	go func() {
		defer b.inProcess.Unlock()
		if err := b.io.MergeAndStore(context.Background(), baseBlockNum, blocksToBundle); err != nil {
			b.bundleError <- err
			return
		}
		if forkableIO, ok := b.io.(ForkAwareIOInterface); ok {
			forkableIO.MoveForkedBlocks(context.Background(), forkedBlocks)
		}
		// we do not delete bundled blocks here, they get pruned later. keeping the blocks from the last bundle is useful for bootstrapping
	}()

	b.Lock()
	// we keep the last block of the bundle, only deleting it on next merge, to facilitate joining to one-block-filled hub
	lastBlock := b.irreversibleBlocks[len(b.irreversibleBlocks)-1]
	b.irreversibleBlocks = []*bstream.OneBlockFile{lastBlock, obf}
	b.baseBlockNum += b.bundleSize
	for obf.Num > b.baseBlockNum+b.bundleSize { // skip more merged-block-files
		b.inProcess.Lock()
		if err := b.io.MergeAndStore(context.Background(), b.baseBlockNum, []*bstream.OneBlockFile{lastBlock}); err != nil { // lastBlock will be excluded from bundle but is useful to bundler
			return err
		}
		b.inProcess.Unlock()
		b.baseBlockNum += b.bundleSize
	}
	b.Unlock()

	if b.stopBlock != 0 && b.baseBlockNum >= b.stopBlock {
		return ErrStopBlockReached
	}

	return nil
}

// String can be called from a different thread
func (b *Bundler) String() string {
	b.Lock()
	defer b.Unlock()

	var firstBlock, lastBlock string
	length := len(b.irreversibleBlocks)
	if length != 0 {
		firstBlock = b.irreversibleBlocks[0].String()
		lastBlock = b.irreversibleBlocks[length-1].String()
	}

	return fmt.Sprintf(
		"bundle_size: %d, base_block_num: %d, first_block: %s, last_block: %s, length: %d",
		b.bundleSize,
		b.baseBlockNum,
		firstBlock,
		lastBlock,
		length,
	)
}
