package apps

import (
	"time"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
	firecore "github.com/streamingfast/firehose-core"
	"github.com/streamingfast/firehose-core/launcher"
	"github.com/streamingfast/firehose-core/relayer/app/relayer"
	"go.uber.org/zap"
)

func RegisterRelayerApp(rootLog *zap.Logger) {
	launcher.RegisterApp(rootLog, &launcher.AppDef{
		ID:          "relayer",
		Title:       "Relayer",
		Description: "Serves blocks as a stream, with a buffer",
		RegisterFlags: func(cmd *cobra.Command) error {
			cmd.Flags().String("relayer-grpc-listen-addr", firecore.RelayerServingAddr, "Address to listen for incoming gRPC requests")
			cmd.Flags().StringSlice("relayer-source", []string{firecore.ReaderNodeGRPCAddr}, "List of live sources (reader(s)) to connect to for live block feeds (repeat flag as needed)")
			cmd.Flags().Duration("relayer-max-source-latency", 999999*time.Hour, "Max latency tolerated to connect to a source. A performance optimization for when you have redundant sources and some may not have caught up")
			return nil
		},
		FactoryFunc: func(runtime *launcher.Runtime) (launcher.App, error) {
			sfDataDir := runtime.AbsDataDir

			return relayer.New(&relayer.Config{
				SourcesAddr:      viper.GetStringSlice("relayer-source"),
				OneBlocksURL:     firecore.MustReplaceDataDir(sfDataDir, viper.GetString("common-one-block-store-url")),
				GRPCListenAddr:   viper.GetString("relayer-grpc-listen-addr"),
				MaxSourceLatency: viper.GetDuration("relayer-max-source-latency"),
			}), nil
		},
	})
}
