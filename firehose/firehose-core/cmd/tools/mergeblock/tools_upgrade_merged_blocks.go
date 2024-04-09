package mergeblock

import (
	"context"
	"errors"
	"fmt"
	"io"
	"strconv"

	"github.com/spf13/cobra"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/streamingfast/bstream/stream"
	"github.com/streamingfast/dstore"
	firecore "github.com/streamingfast/firehose-core"
	"go.uber.org/zap"
)

func NewToolsUpgradeMergedBlocksCmd[B firecore.Block](chain *firecore.Chain[B], rootLog *zap.Logger) *cobra.Command {
	return &cobra.Command{
		Use:   "upgrade-merged-blocks <source> <destination> <range>",
		Short: "From a merged-blocks source, rewrite blocks to a new merged-blocks destination, while applying all possible upgrades",
		Args:  cobra.ExactArgs(4),
		RunE:  getMergedBlockUpgrader(chain.Tools.MergedBlockUpgrader, rootLog),
	}
}

func getMergedBlockUpgrader(tweakFunc func(block *pbbstream.Block) (*pbbstream.Block, error), rootLog *zap.Logger) func(cmd *cobra.Command, args []string) error {
	return func(cmd *cobra.Command, args []string) error {
		source := args[0]
		sourceStore, err := dstore.NewDBinStore(source)
		if err != nil {
			return fmt.Errorf("reading source store: %w", err)
		}

		dest := args[1]
		destStore, err := dstore.NewStore(dest, "dbin.zst", "zstd", true)
		if err != nil {
			return fmt.Errorf("reading destination store: %w", err)
		}

		start, err := strconv.ParseUint(args[2], 10, 64)
		if err != nil {
			return fmt.Errorf("parsing start block num: %w", err)
		}
		stop, err := strconv.ParseUint(args[3], 10, 64)
		if err != nil {
			return fmt.Errorf("parsing stop block num: %w", err)
		}

		rootLog.Info("starting block upgrader process", zap.Uint64("start", start), zap.Uint64("stop", stop), zap.String("source", source), zap.String("dest", dest))
		writer := &firecore.MergedBlocksWriter{
			Cmd:          cmd,
			Store:        destStore,
			LowBlockNum:  firecore.LowBoundary(start),
			StopBlockNum: stop,
			TweakBlock:   tweakFunc,
		}
		stream := stream.New(nil, sourceStore, nil, int64(start), writer, stream.WithFinalBlocksOnly())

		err = stream.Run(context.Background())
		if errors.Is(err, io.EOF) {
			rootLog.Info("Complete!")
			return nil
		}
		return err
	}
}
