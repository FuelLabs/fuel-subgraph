package apps

import (
	"time"

	firecore "github.com/streamingfast/firehose-core"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
	"github.com/streamingfast/firehose-core/launcher"
	"github.com/streamingfast/firehose-core/merger/app/merger"
	"go.uber.org/zap"
)

func RegisterMergerApp(rootLog *zap.Logger) {
	launcher.RegisterApp(rootLog, &launcher.AppDef{
		ID:          "merger",
		Title:       "Merger",
		Description: "Produces merged block files from single-block files",
		RegisterFlags: func(cmd *cobra.Command) error {
			cmd.Flags().String("merger-grpc-listen-addr", firecore.MergerServingAddr, "Address to listen for incoming gRPC requests")
			cmd.Flags().Uint64("merger-prune-forked-blocks-after", 50000, "Number of blocks that must pass before we delete old forks (one-block-files lingering)")
			cmd.Flags().Uint64("merger-stop-block", 0, "If non-zero, merger will trigger shutdown when blocks have been merged up to this block")
			cmd.Flags().Duration("merger-time-between-store-lookups", 1*time.Second, "Delay between source store polling (should be higher for remote storage)")
			cmd.Flags().Duration("merger-time-between-store-pruning", time.Minute, "Delay between source store pruning loops")
			cmd.Flags().Int("merger-delete-threads", 8, "Number of threads for deleting files in parallel (increase this in case the merger isn't able to keep up with deleting one-block files).")
			return nil
		},
		FactoryFunc: func(runtime *launcher.Runtime) (launcher.App, error) {
			mergedBlocksStoreURL, oneBlocksStoreURL, forkedBlocksStoreURL, err := firecore.GetCommonStoresURLs(runtime.AbsDataDir)
			if err != nil {
				return nil, err
			}

			return merger.New(&merger.Config{
				GRPCListenAddr:               viper.GetString("merger-grpc-listen-addr"),
				PruneForkedBlocksAfter:       viper.GetUint64("merger-prune-forked-blocks-after"),
				StorageOneBlockFilesPath:     oneBlocksStoreURL,
				StorageMergedBlocksFilesPath: mergedBlocksStoreURL,
				StorageForkedBlocksFilesPath: forkedBlocksStoreURL,
				StopBlock:                    viper.GetUint64("merger-stop-block"),
				TimeBetweenPruning:           viper.GetDuration("merger-time-between-store-pruning"),
				TimeBetweenPolling:           viper.GetDuration("merger-time-between-store-lookups"),
				FilesDeleteThreads:           viper.GetInt("merger-delete-threads"),
			}), nil
		},
	})
}
