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

	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/dstore"
	"github.com/streamingfast/logging"
	"github.com/streamingfast/shutter"
	"go.uber.org/zap"
)

type Archiver struct {
	*shutter.Shutter

	startBlock     uint64
	oneblockSuffix string

	localOneBlocksStore dstore.Store

	fileUploader *FileUploader
	logger       *zap.Logger
	tracer       logging.Tracer
}

func NewArchiver(
	startBlock uint64,
	oneblockSuffix string,
	localOneBlocksStore dstore.Store,
	remoteOneBlocksStore dstore.Store,
	logger *zap.Logger,
	tracer logging.Tracer,
) *Archiver {

	fileUploader := NewFileUploader(
		localOneBlocksStore,
		remoteOneBlocksStore,
		logger)

	a := &Archiver{
		Shutter:             shutter.New(),
		startBlock:          startBlock,
		oneblockSuffix:      oneblockSuffix,
		localOneBlocksStore: localOneBlocksStore,
		fileUploader:        fileUploader,
		logger:              logger,
		tracer:              tracer,
	}

	return a
}

func (a *Archiver) Start(ctx context.Context) {
	a.OnTerminating(func(err error) {
		a.logger.Info("archiver selector is terminating", zap.Error(err))
	})

	a.OnTerminated(func(err error) {
		a.logger.Info("archiver selector is terminated", zap.Error(err))
	})
	go a.fileUploader.Start(ctx)
}

func (a *Archiver) StoreBlock(ctx context.Context, block *pbbstream.Block) error {
	if block.Number < a.startBlock {
		a.logger.Debug("skipping block below start_block", zap.Uint64("block_num", block.Number), zap.Uint64("start_block", a.startBlock))
		return nil
	}

	pipeRead, pipeWrite := io.Pipe()

	// We are in a pipe context and `a.blockWriterFactory.New(pipeWrite)` writes some bytes to the writer when called.
	// To avoid blocking everything, we must start reading bytes in a goroutine first to ensure the called is not block
	// forever because nobody is reading the pipe.
	writeObjectErrChan := make(chan error)
	go func() {
		writeObjectErrChan <- a.localOneBlocksStore.WriteObject(ctx, bstream.BlockFileNameWithSuffix(block, a.oneblockSuffix), pipeRead)
	}()

	blockWriter, err := bstream.NewDBinBlockWriter(pipeWrite)
	if err != nil {
		return fmt.Errorf("write block factory: %w", err)
	}

	// If `blockWriter.Write()` emits `nil`, the fact that we close with a `nil` error will actually forwards
	// `io.EOF` to the `pipeRead` (e.g. our `WriteObject` call above) which is what we want. If it emits a non
	// `nil`, it will be forwarded to the `pipeRead` which is also correct.
	pipeWrite.CloseWithError(blockWriter.Write(block))

	// We are in a pipe context here, wait until the `WriteObject` call has finished
	err = <-writeObjectErrChan
	if err != nil {
		return err
	}

	return nil
}
