package firehose

import (
	"context"
	"errors"
	"fmt"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/bstream/hub"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/streamingfast/derr"
	"github.com/streamingfast/dmetering"
	"github.com/streamingfast/dstore"
	"go.uber.org/zap"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

type BlockGetter struct {
	mergedBlocksStore dstore.Store
	forkedBlocksStore dstore.Store
	hub               *hub.ForkableHub
}

func NewBlockGetter(
	mergedBlocksStore dstore.Store,
	forkedBlocksStore dstore.Store,
	hub *hub.ForkableHub,
) *BlockGetter {
	return &BlockGetter{
		mergedBlocksStore: mergedBlocksStore,
		forkedBlocksStore: forkedBlocksStore,
		hub:               hub,
	}
}

func (g *BlockGetter) Get(
	ctx context.Context,
	num uint64,
	id string,
	logger *zap.Logger) (out *pbbstream.Block, err error) {

	id = bstream.NormalizeBlockID(id)
	reqLogger := logger.With(
		zap.Uint64("num", num),
		zap.String("id", id),
	)

	// check for block in live segment: Hub
	if g.hub != nil && num > g.hub.LowestBlockNum() {
		if blk := g.hub.GetBlock(num, id); blk != nil {
			reqLogger.Info("single block request", zap.String("source", "hub"), zap.Bool("found", true))
			return blk, nil
		}
		reqLogger.Info("single block request", zap.String("source", "hub"), zap.Bool("found", false))
		return nil, status.Error(codes.NotFound, "live block not found in hub")
	}

	mergedBlocksStore := g.mergedBlocksStore
	if clonable, ok := mergedBlocksStore.(dstore.Clonable); ok {
		var err error
		mergedBlocksStore, err = clonable.Clone(ctx)
		if err != nil {
			return nil, err
		}
		mergedBlocksStore.SetMeter(dmetering.GetBytesMeter(ctx))
	}

	// check for block in mergedBlocksStore
	err = derr.RetryContext(ctx, 3, func(ctx context.Context) error {
		blk, err := bstream.FetchBlockFromMergedBlocksStore(ctx, num, mergedBlocksStore)
		if err != nil {
			if errors.Is(err, dstore.ErrNotFound) {
				return derr.NewFatalError(err)
			}
			return err
		}
		if id == "" || blk.Id == id {
			reqLogger.Info("single block request", zap.String("source", "merged_blocks"), zap.Bool("found", true))
			out = blk
			return nil
		}
		return derr.NewFatalError(fmt.Errorf("wrong block: found %s, expecting %s", blk.Id, id))
	})
	if out != nil {
		return out, nil
	}

	// check for block in forkedBlocksStore
	if g.forkedBlocksStore != nil {
		forkedBlocksStore := g.forkedBlocksStore
		if clonable, ok := forkedBlocksStore.(dstore.Clonable); ok {
			var err error
			forkedBlocksStore, err = clonable.Clone(ctx)
			if err != nil {
				return nil, err
			}
			forkedBlocksStore.SetMeter(dmetering.GetBytesMeter(ctx))
		}

		if blk, _ := bstream.FetchBlockFromOneBlockStore(ctx, num, id, forkedBlocksStore); blk != nil {
			reqLogger.Info("single block request", zap.String("source", "forked_blocks"), zap.Bool("found", true))
			return blk, nil
		}
	}

	reqLogger.Info("single block request", zap.Bool("found", false), zap.Error(err))
	return nil, status.Error(codes.NotFound, "block not found in files")
}
