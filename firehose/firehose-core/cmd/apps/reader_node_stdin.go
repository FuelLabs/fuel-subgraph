// Copyright 2021 dfuse Platform Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package apps

import (
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
	firecore "github.com/streamingfast/firehose-core"
	"github.com/streamingfast/firehose-core/launcher"
	nodeManager "github.com/streamingfast/firehose-core/node-manager"
	nodeReaderStdinApp "github.com/streamingfast/firehose-core/node-manager/app/node_reader_stdin"
	"github.com/streamingfast/firehose-core/node-manager/metrics"
	"github.com/streamingfast/firehose-core/node-manager/mindreader"
	"github.com/streamingfast/logging"
	"go.uber.org/zap"
)

func RegisterReaderNodeStdinApp[B firecore.Block](chain *firecore.Chain[B], rootLog *zap.Logger) {
	appLogger, appTracer := logging.PackageLogger("reader-node-stdin", chain.LoggerPackageID("reader-node-stdin"))

	launcher.RegisterApp(rootLog, &launcher.AppDef{
		ID:            "reader-node-stdin",
		Title:         "Reader Node (stdin)",
		Description:   "Blocks reading node, unmanaged, reads Firehose logs from standard input and transform them into Firehose chain specific blocks",
		RegisterFlags: func(cmd *cobra.Command) error { return nil },
		FactoryFunc: func(runtime *launcher.Runtime) (launcher.App, error) {
			sfDataDir := runtime.AbsDataDir
			archiveStoreURL := firecore.MustReplaceDataDir(sfDataDir, viper.GetString("common-one-block-store-url"))
			consoleReaderFactory := func(lines chan string) (mindreader.ConsolerReader, error) {
				return chain.ConsoleReaderFactory(lines, chain.BlockEncoder, appLogger, appTracer)
			}

			metricID := "reader-node-stdin"
			headBlockTimeDrift := metrics.NewHeadBlockTimeDrift(metricID)
			headBlockNumber := metrics.NewHeadBlockNumber(metricID)
			appReadiness := metrics.NewAppReadiness(metricID)
			metricsAndReadinessManager := nodeManager.NewMetricsAndReadinessManager(headBlockTimeDrift, headBlockNumber, appReadiness, viper.GetDuration("reader-node-readiness-max-latency"))

			return nodeReaderStdinApp.New(&nodeReaderStdinApp.Config{
				GRPCAddr:                   viper.GetString("reader-node-grpc-listen-addr"),
				OneBlocksStoreURL:          archiveStoreURL,
				MindReadBlocksChanCapacity: viper.GetInt("reader-node-blocks-chan-capacity"),
				StartBlockNum:              viper.GetUint64("reader-node-start-block-num"),
				StopBlockNum:               viper.GetUint64("reader-node-stop-block-num"),
				WorkingDir:                 firecore.MustReplaceDataDir(sfDataDir, viper.GetString("reader-node-working-dir")),
				OneBlockSuffix:             viper.GetString("reader-node-one-block-suffix"),
			}, &nodeReaderStdinApp.Modules{
				ConsoleReaderFactory:       consoleReaderFactory,
				MetricsAndReadinessManager: metricsAndReadinessManager,
			}, appLogger, appTracer), nil
		},
	})
}
