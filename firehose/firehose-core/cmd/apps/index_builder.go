package apps

import (
	"context"
	"fmt"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
	"github.com/streamingfast/bstream"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	bstransform "github.com/streamingfast/bstream/transform"
	firecore "github.com/streamingfast/firehose-core"
	index_builder "github.com/streamingfast/firehose-core/index-builder/app/index-builder"
	"github.com/streamingfast/firehose-core/launcher"
	"go.uber.org/zap"
)

func RegisterIndexBuilderApp[B firecore.Block](chain *firecore.Chain[B], rootLog *zap.Logger) {
	launcher.RegisterApp(rootLog, &launcher.AppDef{
		ID:          "index-builder",
		Title:       "Index Builder",
		Description: "App the builds indexes out of Firehose blocks",
		RegisterFlags: func(cmd *cobra.Command) error {
			cmd.Flags().String("index-builder-grpc-listen-addr", firecore.IndexBuilderServiceAddr, "Address to listen for grpc-based healthz check")
			cmd.Flags().Uint64("index-builder-index-size", 10000, "Size of index bundles that will be created")
			cmd.Flags().Uint64("index-builder-start-block", 0, "Block number to start indexing")
			cmd.Flags().Uint64("index-builder-stop-block", 0, "Block number to stop indexing")
			return nil
		},
		InitFunc: func(runtime *launcher.Runtime) error {
			return nil
		},
		FactoryFunc: func(runtime *launcher.Runtime) (launcher.App, error) {
			mergedBlocksStoreURL, _, _, err := firecore.GetCommonStoresURLs(runtime.AbsDataDir)
			if err != nil {
				return nil, err
			}

			indexStore, lookupIdxSizes, err := firecore.GetIndexStore(runtime.AbsDataDir)
			if err != nil {
				return nil, err
			}

			// Chain must have been validated, so if we are here, it's because there is
			// exactly one index in BlockIndexerFactories map.
			indexShortName, indexerFactory, found := getMapFirst(chain.BlockIndexerFactories)
			if !found {
				return nil, fmt.Errorf("no indexer factory found but one should be defined at this point")
			}

			startBlockResolver := func(ctx context.Context) (uint64, error) {
				select {
				case <-ctx.Done():
					return 0, ctx.Err()
				default:
				}

				startBlockNum := bstransform.FindNextUnindexed(
					ctx,
					viper.GetUint64("index-builder-start-block"),
					lookupIdxSizes,
					indexShortName,
					indexStore,
				)

				return startBlockNum, nil
			}
			stopBlockNum := viper.GetUint64("index-builder-stop-block")

			indexer, err := indexerFactory(indexStore, viper.GetUint64("index-builder-index-size"))
			if err != nil {
				return nil, fmt.Errorf("unable to create indexer: %w", err)
			}

			handler := bstream.HandlerFunc(func(blk *pbbstream.Block, _ interface{}) error {
				var b = chain.BlockFactory()
				if err := blk.Payload.UnmarshalTo(b); err != nil {
					return err
				}
				return indexer.ProcessBlock(any(b).(B))
			})

			app := index_builder.New(&index_builder.Config{
				BlockHandler:         handler,
				StartBlockResolver:   startBlockResolver,
				EndBlock:             stopBlockNum,
				MergedBlocksStoreURL: mergedBlocksStoreURL,
				GRPCListenAddr:       viper.GetString("index-builder-grpc-listen-addr"),
			})

			return app, nil
		},
	})
}

func getMapFirst[K comparable, V any](m map[K]V) (k K, v V, found bool) {
	for k := range m {
		return k, m[k], true
	}

	return k, v, false
}
