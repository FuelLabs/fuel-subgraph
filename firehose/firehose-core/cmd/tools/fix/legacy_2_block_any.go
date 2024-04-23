package fix

import (
	"fmt"
	"io"

	"github.com/spf13/cobra"
	"github.com/streamingfast/bstream"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/streamingfast/dstore"
	firecore "github.com/streamingfast/firehose-core"
	"github.com/streamingfast/firehose-core/cmd/tools/check"
	"github.com/streamingfast/firehose-core/types"
	"go.uber.org/zap"
)

func NewLegacy2BlockAby[B firecore.Block](chain *firecore.Chain[B], zlog *zap.Logger) *cobra.Command {
	return &cobra.Command{
		Use:   "legacy_2_block_any <src_merged_blocks_store> <dest_one_blocks_store> [<block_range>]",
		Short: "converts merged blocks from legacy format to block any format",
		Args:  cobra.ExactArgs(3),
		RunE:  runLegacy2BlockAnyE(zlog),
	}
}

func runLegacy2BlockAnyE(zlog *zap.Logger) firecore.CommandExecutor {
	return func(cmd *cobra.Command, args []string) error {
		ctx := cmd.Context()

		srcStore, err := dstore.NewDBinStore(args[0])
		if err != nil {
			return fmt.Errorf("unable to create source store: %w", err)
		}

		destStore, err := dstore.NewDBinStore(args[1])
		if err != nil {
			return fmt.Errorf("unable to create destination store: %w", err)
		}

		blockRange, err := types.GetBlockRangeFromArg(args[2])
		if err != nil {
			return fmt.Errorf("parsing block range: %w", err)
		}

		err = srcStore.Walk(ctx, check.WalkBlockPrefix(blockRange, 100), func(filename string) error {
			zlog.Debug("checking merged block file", zap.String("filename", filename))

			startBlock := firecore.MustParseUint64(filename)

			if startBlock > uint64(blockRange.GetStopBlockOr(firecore.MaxUint64)) {
				zlog.Debug("skipping merged block file", zap.String("reason", "past stop block"), zap.String("filename", filename))
				return dstore.StopIteration
			}

			if startBlock+100 < uint64(blockRange.Start) {
				zlog.Debug("skipping merged block file", zap.String("reason", "before start block"), zap.String("filename", filename))
				return nil
			}

			rc, err := srcStore.OpenObject(ctx, filename)
			if err != nil {
				return fmt.Errorf("failed to open %s: %w", filename, err)
			}
			defer rc.Close()

			br, err := bstream.NewDBinBlockReader(rc)
			if err != nil {
				return fmt.Errorf("creating block reader: %w", err)
			}

			mergeWriter := &firecore.MergedBlocksWriter{
				Store: destStore,
				TweakBlock: func(b *pbbstream.Block) (*pbbstream.Block, error) {

					return b, nil
				},
				Logger: zlog,
			}

			seen := make(map[string]bool)

			var lastBlockID string
			var lastBlockNum uint64

			// iterate through the blocks in the file
			for {
				block, err := br.Read()
				if err == io.EOF {
					break
				}

				if block.Number < startBlock {
					continue
				}

				if block.Number > blockRange.GetStopBlockOr(firecore.MaxUint64) {
					break
				}

				if seen[block.Id] {
					zlog.Info("skipping seen block (source merged-blocks had duplicates, skipping)", zap.String("id", block.Id), zap.Uint64("num", block.Number))
					continue
				}

				if lastBlockID != "" && block.ParentId != lastBlockID {
					return fmt.Errorf("got an invalid sequence of blocks: block %q has previousId %s, previous block %d had ID %q, this endpoint is serving blocks out of order", block.String(), block.ParentId, lastBlockNum, lastBlockID)
				}
				lastBlockID = block.Id
				lastBlockNum = block.Number

				seen[block.Id] = true

				if err := mergeWriter.ProcessBlock(block, nil); err != nil {
					return fmt.Errorf("write to blockwriter: %w", err)
				}
			}

			return nil
		})

		if err == io.EOF {
			return nil
		}

		if err != nil {
			return err
		}

		return nil
	}
}
