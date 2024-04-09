package mergeblock

import (
	"fmt"
	"io"

	"github.com/streamingfast/firehose-core/cmd/tools/check"
	"github.com/streamingfast/firehose-core/types"

	"github.com/spf13/cobra"
	"github.com/streamingfast/bstream"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/streamingfast/dstore"
	firecore "github.com/streamingfast/firehose-core"
	"go.uber.org/zap"
)

func NewToolsUnmergeBlocksCmd[B firecore.Block](chain *firecore.Chain[B], zlog *zap.Logger) *cobra.Command {
	return &cobra.Command{
		Use:   "unmerge-blocks <src_merged_blocks_store> <dest_one_blocks_store> [<block_range>]",
		Short: "Unmerges merged block files into one-block-files",
		Args:  cobra.ExactArgs(3),
		RunE:  runUnmergeBlocksE(zlog),
	}
}

func runUnmergeBlocksE(zlog *zap.Logger) firecore.CommandExecutor {
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

			// iterate through the blocks in the file
			for {
				block, err := br.Read()
				if err == io.EOF {
					break
				}

				if block.Number < uint64(blockRange.Start) {
					continue
				}

				if block.Number > blockRange.GetStopBlockOr(firecore.MaxUint64) {
					break
				}

				oneblockFilename := bstream.BlockFileNameWithSuffix(block, "extracted")
				zlog.Debug("writing block", zap.Uint64("block_num", block.Number), zap.String("filename", oneblockFilename))

				pr, pw := io.Pipe()

				//write block data to pipe, and then close to signal end of data
				go func(block *pbbstream.Block) {
					var err error
					defer func() {
						pw.CloseWithError(err)
					}()

					bw, err := bstream.NewDBinBlockWriter(pw)
					if err != nil {
						zlog.Error("creating block writer", zap.Error(err))
						return
					}

					err = bw.Write(block)
					if err != nil {
						zlog.Error("writing block", zap.Error(err))
						return
					}
				}(block)

				//read block data from pipe and write block data to dest store
				err = destStore.WriteObject(ctx, oneblockFilename, pr)
				if err != nil {
					return fmt.Errorf("writing block %d to %s: %w", block.Number, oneblockFilename, err)
				}

				zlog.Info("wrote block", zap.Uint64("block_num", block.Number), zap.String("filename", oneblockFilename))
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
