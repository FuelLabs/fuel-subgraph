package index_builder

import (
	"context"
	"errors"
	"fmt"

	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	firecore "github.com/streamingfast/firehose-core"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/bstream/stream"
	"github.com/streamingfast/dstore"
	"github.com/streamingfast/firehose-core/index-builder/metrics"
	pbfirehose "github.com/streamingfast/pbgo/sf/firehose/v2"
	"github.com/streamingfast/shutter"
	"go.uber.org/zap"
)

type IndexBuilder struct {
	*shutter.Shutter
	logger *zap.Logger

	startBlockNum uint64
	stopBlockNum  uint64

	handler bstream.Handler

	blocksStore dstore.Store
}

func NewIndexBuilder(logger *zap.Logger, handler bstream.Handler, startBlockNum, stopBlockNum uint64, blockStore dstore.Store) *IndexBuilder {
	return &IndexBuilder{
		Shutter:       shutter.New(),
		startBlockNum: startBlockNum,
		stopBlockNum:  stopBlockNum,
		handler:       handler,
		blocksStore:   blockStore,

		logger: logger,
	}
}

func (app *IndexBuilder) Launch() {
	err := app.launch()
	if errors.Is(err, stream.ErrStopBlockReached) {
		app.logger.Info("index builder reached stop block", zap.Uint64("stop_block_num", app.stopBlockNum))
		err = nil
	}
	app.logger.Info("index builder exited", zap.Error(err))
	app.Shutdown(err)
}

func (app *IndexBuilder) launch() error {
	startBlockNum := app.startBlockNum
	stopBlockNum := app.stopBlockNum

	streamFactory := firecore.NewStreamFactory(
		app.blocksStore,
		nil,
		nil,
		nil,
	)
	ctx := context.Background()

	req := &pbfirehose.Request{
		StartBlockNum:   int64(startBlockNum),
		StopBlockNum:    stopBlockNum,
		FinalBlocksOnly: true,
	}

	handlerFunc := func(block *pbbstream.Block, obj interface{}) error {
		app.logger.Debug("handling block", zap.Uint64("block_num", block.Number))

		metrics.HeadBlockNumber.SetUint64(block.Number)
		metrics.HeadBlockTimeDrift.SetBlockTime(block.Time())
		metrics.AppReadiness.SetReady()

		app.logger.Debug("updated head block metrics", zap.Uint64("block_num", block.Number), zap.Time("block_time", block.Time()))

		return app.handler.ProcessBlock(block, obj)
	}

	stream, err := streamFactory.New(
		ctx,
		bstream.HandlerFunc(handlerFunc),
		req,
		app.logger,
	)

	if err != nil {
		return fmt.Errorf("getting firehose stream: %w", err)
	}

	return stream.Run(ctx)
}
