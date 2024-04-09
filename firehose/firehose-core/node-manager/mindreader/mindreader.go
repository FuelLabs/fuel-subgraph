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

package mindreader

import (
	"context"
	"fmt"
	"io"
	"os"
	"path"
	"regexp"
	"sync"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/bstream/blockstream"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/streamingfast/dstore"
	"github.com/streamingfast/firehose-core/internal/utils"
	nodeManager "github.com/streamingfast/firehose-core/node-manager"
	"github.com/streamingfast/logging"
	"github.com/streamingfast/shutter"
	"go.uber.org/zap"
)

var (
	oneblockSuffixRegexp = regexp.MustCompile(`^[\w\-]+$`)
)

type ConsolerReader interface {
	ReadBlock() (obj *pbbstream.Block, err error)
	Done() <-chan interface{}
}

type CloseableConsoleReader interface {
	ConsolerReader

	Close() error
}

type ConsolerReaderFactory func(lines chan string) (ConsolerReader, error)

type MindReaderPlugin struct {
	*shutter.Shutter

	archiver                 *Archiver // transformed blocks are sent to Archiver
	consoleReaderFactory     ConsolerReaderFactory
	stopBlock                uint64 // if set, call shutdownFunc(nil) when we hit this number
	channelCapacity          int    // transformed blocks are buffered in a channel
	forceFinalityAfterBlocks *uint64

	lastSeenBlock     bstream.BlockRef
	lastSeenBlockLock sync.RWMutex

	headBlockUpdater  nodeManager.HeadBlockUpdater
	onBlockWritten    nodeManager.OnBlockWritten
	blockStreamServer *blockstream.Server
	zlogger           *zap.Logger

	lines               chan string
	consoleReader       ConsolerReader // contains the 'reader' part of the pipe
	consumeReadFlowDone chan interface{}
}

// NewMindReaderPlugin initiates its own:
// * ConsoleReader (from given Factory)
// * Archiver (from archive store params)
// * Shutter
func NewMindReaderPlugin(
	oneBlocksStoreURL string,
	workingDirectory string,
	consoleReaderFactory ConsolerReaderFactory,
	startBlockNum uint64,
	stopBlockNum uint64,
	channelCapacity int,
	headBlockUpdater nodeManager.HeadBlockUpdater,
	shutdownFunc func(error),
	oneBlockSuffix string,
	blockStreamServer *blockstream.Server,
	zlogger *zap.Logger,
	tracer logging.Tracer,
) (*MindReaderPlugin, error) {
	err := validateOneBlockSuffix(oneBlockSuffix)
	if err != nil {
		return nil, err
	}

	zlogger.Info("creating mindreader plugin",
		zap.String("one_blocks_store_url", oneBlocksStoreURL),
		zap.String("one_block_suffix", oneBlockSuffix),
		zap.String("working_directory", workingDirectory),
		zap.Uint64("start_block_num", startBlockNum),
		zap.Uint64("stop_block_num", stopBlockNum),
		zap.Int("channel_capacity", channelCapacity),
		zap.Bool("with_head_block_updater", headBlockUpdater != nil),
		zap.Bool("with_shutdown_func", shutdownFunc != nil),
	)

	// Create directory and its parent(s), it's a no-op if everything already exists
	err = os.MkdirAll(workingDirectory, os.ModePerm)
	if err != nil {
		return nil, fmt.Errorf("create working directory: %w", err)
	}

	// local store
	localOnBlocksStoreURL := path.Join(workingDirectory, "uploadable-oneblock")
	localOneBlocksStore, err := dstore.NewStore(localOnBlocksStoreURL, "dbin", "", false)
	if err != nil {
		return nil, fmt.Errorf("new local one block store: %w", err)
	}

	remoteOneBlocksStore, err := dstore.NewStore(oneBlocksStoreURL, "dbin.zst", "zstd", false)
	if err != nil {
		return nil, fmt.Errorf("new remote one block store: %w", err)
	}

	archiver := NewArchiver(
		startBlockNum,
		oneBlockSuffix,
		localOneBlocksStore,
		remoteOneBlocksStore,
		zlogger,
		tracer,
	)

	zlogger.Info("creating new mindreader plugin")
	return &MindReaderPlugin{
		Shutter:                  shutter.New(),
		archiver:                 archiver,
		consoleReaderFactory:     consoleReaderFactory,
		stopBlock:                stopBlockNum,
		channelCapacity:          channelCapacity,
		headBlockUpdater:         headBlockUpdater,
		blockStreamServer:        blockStreamServer,
		forceFinalityAfterBlocks: utils.GetEnvForceFinalityAfterBlocks(),
		zlogger:                  zlogger,
	}, nil
}

// Other components may have issues finding the one block files if suffix is invalid
func validateOneBlockSuffix(suffix string) error {
	if suffix == "" {
		return fmt.Errorf("oneblock_suffix cannot be empty")
	}
	if !oneblockSuffixRegexp.MatchString(suffix) {
		return fmt.Errorf("oneblock_suffix contains invalid characters: %q", suffix)
	}
	return nil
}

func (p *MindReaderPlugin) Name() string {
	return "MindReaderPlugin"
}

func (p *MindReaderPlugin) Launch() {
	ctx, cancel := context.WithCancel(context.Background())
	p.OnTerminating(func(_ error) {
		cancel()
	})

	p.zlogger.Info("starting mindreader")

	p.consumeReadFlowDone = make(chan interface{})

	lines := make(chan string, 10000) //need a config here?
	p.lines = lines

	consoleReader, err := p.consoleReaderFactory(lines)
	if err != nil {
		p.Shutdown(err)
	}

	p.consoleReader = consoleReader
	if closer, ok := consoleReader.(CloseableConsoleReader); ok {
		p.OnTerminating(func(_ error) { closer.Close() })
	}

	p.zlogger.Debug("starting archiver")
	p.archiver.Start(ctx)
	p.launch()

}
func (p *MindReaderPlugin) launch() {
	blocks := make(chan *pbbstream.Block, p.channelCapacity)
	p.zlogger.Info("launching blocks reading loop", zap.Int("capacity", p.channelCapacity))
	go p.consumeReadFlow(blocks)

	go func() {
		for {
			err := p.readOneMessage(blocks)
			if err != nil {
				if err == io.EOF {
					p.zlogger.Info("reached end of console reader stream, nothing more to do")
					close(blocks)
					return
				}
				p.zlogger.Error("reading from console logs", zap.Error(err))
				p.Shutdown(err)
				// Always read messages otherwise you'll stall the shutdown lifecycle of the managed process, leading to corrupted database if exit uncleanly afterward
				p.drainMessages()
				close(blocks)
				return
			}
		}
	}()
}

func (p MindReaderPlugin) Stop() {
	p.zlogger.Info("mindreader is stopping")
	if p.lines == nil {
		// If the `lines` channel was not created yet, it means everything was shut down very rapidly
		// and means MindreaderPlugin has not launched yet. Since it has not launched yet, there is
		// no point in waiting for the read flow to complete since the read flow never started. So
		// we exit right now.
		return
	}

	p.Shutdown(nil)

	close(p.lines)
	p.waitForReadFlowToComplete()
}

func (p *MindReaderPlugin) waitForReadFlowToComplete() {
	p.zlogger.Info("waiting until consume read flow (i.e. blocks) is actually done processing blocks...")
	<-p.consumeReadFlowDone
	p.zlogger.Info("consume read flow terminate")
}

// consumeReadFlow is the one function blocking termination until consumption/writeBlock/upload is done
func (p *MindReaderPlugin) consumeReadFlow(blocks <-chan *pbbstream.Block) {
	p.zlogger.Info("starting consume flow")
	defer close(p.consumeReadFlowDone)

	ctx := context.Background()
	for {
		p.zlogger.Debug("waiting to consume next block")
		block, ok := <-blocks
		if !ok {
			p.zlogger.Info("all blocks in channel were drained, exiting read flow")
			p.archiver.Shutdown(nil)

			<-p.archiver.Terminated()
			p.zlogger.Info("archiver termination code completed")

			return
		}

		p.zlogger.Debug("got one block", zap.Uint64("block_num", block.Number))

		err := p.archiver.StoreBlock(ctx, block)
		if err != nil {
			p.zlogger.Error("failed storing block in archiver, shutting down and trying to send next blocks individually. You will need to reprocess over this range.", zap.Error(err), zap.String("received_block", block.Id), zap.Uint64("received_block_num", block.Number))

			if !p.IsTerminating() {
				go p.Shutdown(fmt.Errorf("archiver store block failed: %w", err))
			}

			continue
		}

		if p.onBlockWritten != nil {
			err = p.onBlockWritten(block)
			if err != nil {
				p.zlogger.Error("onBlockWritten callback failed", zap.Error(err))

				if !p.IsTerminating() {
					go p.Shutdown(fmt.Errorf("onBlockWritten callback failed: %w", err))
				}

				continue
			}
		}

		if p.blockStreamServer != nil {
			err = p.blockStreamServer.PushBlock(block)
			if err != nil {
				p.zlogger.Error("failed passing block to block stream server (this should not happen, shutting down)", zap.Error(err))

				if !p.IsTerminating() {
					go p.Shutdown(fmt.Errorf("block stream push block failed: %w", err))
				}

				continue
			}
		}
	}
}

func (p *MindReaderPlugin) drainMessages() {
	for line := range p.lines {
		_ = line
	}
}

func (p *MindReaderPlugin) readOneMessage(blocks chan<- *pbbstream.Block) error {
	block, err := p.consoleReader.ReadBlock()
	if err != nil {
		return err
	}

	if p.forceFinalityAfterBlocks != nil {
		utils.TweakBlockFinality(block, *p.forceFinalityAfterBlocks)
	}

	if block.Number < bstream.GetProtocolFirstStreamableBlock {
		return nil
	}

	p.lastSeenBlockLock.Lock()
	p.lastSeenBlock = block.AsRef()
	p.lastSeenBlockLock.Unlock()

	if p.headBlockUpdater != nil {
		if err := p.headBlockUpdater(block); err != nil {
			p.zlogger.Info("shutting down because head block updater generated an error", zap.Error(err))

			// We are shutting dow in a separate goroutine because the shutdown signal reaches us back at some point which
			// if we were not on a goroutine, we would dead block with the shutdown pipeline that would wait for us to
			// terminate which would never happen.
			//
			// 0a33f6b578cc4d0b
			go p.Shutdown(err)
		}
	}

	blocks <- block

	if p.stopBlock != 0 && block.Number >= p.stopBlock && !p.IsTerminating() {
		p.zlogger.Info("shutting down because requested end block reached", zap.Stringer("block", block))

		// See comment tagged 0a33f6b578cc4d0b
		go p.Shutdown(nil)
	}

	return nil
}

// LogLine receives log line and write it to "pipe" of the local console reader
func (p *MindReaderPlugin) LogLine(in string) {
	if p.IsTerminating() {
		return
	}

	p.lines <- in
}

func (p *MindReaderPlugin) OnBlockWritten(callback nodeManager.OnBlockWritten) {
	p.onBlockWritten = callback
}

// GetMindreaderLineChannel is a marker method that `superviser.Superviser` uses to determine if
// `logplugin.LogPlugin` is an actual mindreader plugin without depending on the `mindreader`
// package in which case it would create an import cycle.
//
// The `superviser.Superviser` defines `type mindreaderPlugin interface { LastSeenBlockNum() bstream.BlockRef }`
// which is respected. This is a trick to avoid circual dependency in imports.
func (p *MindReaderPlugin) LastSeenBlock() bstream.BlockRef {
	p.lastSeenBlockLock.RLock()
	defer p.lastSeenBlockLock.RUnlock()

	return p.lastSeenBlock
}
