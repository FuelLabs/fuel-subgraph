package firecore

import (
	"context"
	"fmt"
	"io"

	"github.com/spf13/cobra"
	"github.com/streamingfast/bstream"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/streamingfast/dstore"
	"go.uber.org/zap"
)

type MergedBlocksWriter struct {
	Store        dstore.Store
	LowBlockNum  uint64
	StopBlockNum uint64

	blocks []*pbbstream.Block
	Logger *zap.Logger
	Cmd    *cobra.Command

	TweakBlock func(*pbbstream.Block) (*pbbstream.Block, error)
}

func (w *MergedBlocksWriter) ProcessBlock(blk *pbbstream.Block, obj interface{}) error {
	if w.TweakBlock != nil {
		b, err := w.TweakBlock(blk)
		if err != nil {
			return fmt.Errorf("tweaking block: %w", err)
		}
		blk = b
	}

	if w.LowBlockNum == 0 && blk.Number > 99 { // initial block
		if blk.Number%100 != 0 && blk.Number != bstream.GetProtocolFirstStreamableBlock {
			return fmt.Errorf("received unexpected block %s (not a boundary, not the first streamable block %d)", blk, bstream.GetProtocolFirstStreamableBlock)
		}
		w.LowBlockNum = LowBoundary(blk.Number)
		w.Logger.Debug("setting initial boundary to %d upon seeing block %s", zap.Uint64("low_boundary", w.LowBlockNum), zap.Uint64("blk_num", blk.Number))
	}

	if blk.Number > w.LowBlockNum+99 {
		w.Logger.Debug("bundling because we saw block %s from next bundle (%d was not seen, it must not exist on this chain)", zap.Uint64("blk_num", blk.Number), zap.Uint64("last_bundle_block", w.LowBlockNum+99))
		if err := w.WriteBundle(); err != nil {
			return err
		}
	}

	if w.StopBlockNum > 0 && blk.Number >= w.StopBlockNum {
		return io.EOF
	}

	w.blocks = append(w.blocks, blk)

	if blk.Number == w.LowBlockNum+99 {
		w.Logger.Debug("bundling on last bundle block", zap.Uint64("last_bundle_block", w.LowBlockNum+99))
		if err := w.WriteBundle(); err != nil {
			return err
		}
		return nil
	}

	return nil
}

func (w *MergedBlocksWriter) WriteBundle() error {
	file := filename(w.LowBlockNum)
	w.Logger.Info("writing merged file to store (suffix: .dbin.zst)", zap.String("filename", file), zap.Uint64("lowBlockNum", w.LowBlockNum))

	if len(w.blocks) == 0 {
		return fmt.Errorf("no blocks to write to bundle")
	}

	pr, pw := io.Pipe()

	go func() {
		var err error
		defer func() {
			pw.CloseWithError(err)
		}()

		blockWriter, err := bstream.NewDBinBlockWriter(pw)
		if err != nil {
			return
		}

		for _, blk := range w.blocks {
			err = blockWriter.Write(blk)
			if err != nil {
				return
			}
		}
	}()

	err := w.Store.WriteObject(context.Background(), file, pr)
	if err != nil {
		w.Logger.Error("writing to store", zap.Error(err))
	}

	w.LowBlockNum += 100
	w.blocks = nil

	return err
}
func filename(num uint64) string {
	return fmt.Sprintf("%010d", num)
}

func LowBoundary(i uint64) uint64 {
	return i - (i % 100)
}
