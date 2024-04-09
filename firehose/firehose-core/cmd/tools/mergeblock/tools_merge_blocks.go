package mergeblock

import (
	"errors"
	"fmt"
	"io"
	"strconv"

	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"

	"github.com/streamingfast/bstream"

	"github.com/spf13/cobra"
	"github.com/streamingfast/dstore"
	firecore "github.com/streamingfast/firehose-core"
	"go.uber.org/zap"
)

func NewToolsMergeBlocksCmd[B firecore.Block](chain *firecore.Chain[B], zlog *zap.Logger) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "merge-blocks <src_one_blocks_store> <dest_merged_block_store> <low_block_num>",
		Short: "Merges one-block files into merged-block file",
		Args:  cobra.ExactArgs(3),
		RunE:  runMergeBlocksE(zlog),
	}

	cmd.Flags().String("force-block-type", "", "When set, will force the block type to the given value.")

	return cmd
}

func runMergeBlocksE(zlog *zap.Logger) firecore.CommandExecutor {
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

		lowBundary, err := strconv.ParseUint(args[2], 10, 64)
		if err != nil {
			return fmt.Errorf("converting low bundary string to uint64: %w", err)
		}

		mergeWriter := &firecore.MergedBlocksWriter{
			Store:        destStore,
			LowBlockNum:  lowBundary,
			StopBlockNum: 0,
			Logger:       zlog,
			Cmd:          cmd,
		}

		zlog.Info("starting block merger process", zap.String("source", args[0]), zap.String("dest", args[1]))

		var lastFilename string
		var blockCount int
		var previousBlockNumber uint64
		err = srcStore.WalkFrom(ctx, "", fmt.Sprintf("%010d", lowBundary), func(filename string) error {
			var currentBlockNumber uint64
			currentBlockNumber, _, _, _, _, err = bstream.ParseFilename(filename)
			if err != nil {
				return fmt.Errorf("parsing filename %s: %w", filename, err)
			}

			if previousBlockNumber == currentBlockNumber {
				zlog.Warn("skipping duplicate block", zap.String("filename", filename))
				return nil
			}

			if currentBlockNumber > lowBundary+100 {
				return dstore.StopIteration
			}

			var fileReader io.Reader
			fileReader, err = srcStore.OpenObject(ctx, filename)
			if err != nil {
				return fmt.Errorf("creating reader: %w", err)
			}

			var blockReader *bstream.DBinBlockReader
			blockReader, err = bstream.NewDBinBlockReader(fileReader)
			if err != nil {
				return fmt.Errorf("creating block reader: %w", err)
			}

			var currentBlock *pbbstream.Block
			currentBlock, err = blockReader.Read()
			if err != nil {
				return fmt.Errorf("reading block: %w", err)
			}

			if err = mergeWriter.ProcessBlock(currentBlock, nil); err != nil {
				return fmt.Errorf("processing block: %w", err)
			}

			lastFilename = filename
			blockCount += 1

			previousBlockNumber = currentBlockNumber
			return nil
		})

		mergeWriter.Logger = mergeWriter.Logger.With(zap.String("last_filename", lastFilename), zap.Int("block_count", blockCount))
		if err != nil {
			if errors.Is(err, dstore.StopIteration) {
				err = mergeWriter.WriteBundle()
				if err != nil {
					return fmt.Errorf("writing bundle: %w", err)
				}
				fmt.Println("done")
			}
			return fmt.Errorf("walking source store: %w", err)
		}

		err = mergeWriter.WriteBundle()
		if err != nil {
			return fmt.Errorf("writing bundle: %w", err)
		}

		return nil
	}

}
