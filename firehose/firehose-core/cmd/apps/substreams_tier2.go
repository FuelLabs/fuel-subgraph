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
	"fmt"
	"net/url"
	"os"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
	discoveryservice "github.com/streamingfast/dgrpc/server/discovery-service"
	firecore "github.com/streamingfast/firehose-core"
	"github.com/streamingfast/firehose-core/launcher"
	"github.com/streamingfast/logging"
	"github.com/streamingfast/substreams/app"
	"github.com/streamingfast/substreams/wasm"
	"go.uber.org/zap"
)

var ss2HeadBlockNumMetric = metricset.NewHeadBlockNumber("substreams-tier2")
var ss2HeadTimeDriftmetric = metricset.NewHeadTimeDrift("substreams-tier2")

func RegisterSubstreamsTier2App[B firecore.Block](chain *firecore.Chain[B], rootLog *zap.Logger) {
	appLogger, _ := logging.PackageLogger("substreams-tier2", "github.com/streamingfast/firehose-core/firehose-ethereum/substreams-tier2")

	launcher.RegisterApp(rootLog, &launcher.AppDef{
		ID:          "substreams-tier2",
		Title:       "Substreams tier2 server",
		Description: "Provides a substreams grpc endpoint",
		RegisterFlags: func(cmd *cobra.Command) error {
			cmd.Flags().String("substreams-tier2-grpc-listen-addr", firecore.SubstreamsTier2GRPCServingAddr, "Address on which the substreams tier2 will listen. Default is plain-text, appending a '*' to the end to jkkkj")
			cmd.Flags().String("substreams-tier2-discovery-service-url", "", "URL to advertise presence to the grpc discovery service") //traffic-director://xds?vpc_network=vpc-global&use_xds_reds=true
			cmd.Flags().Uint64("substreams-tier2-max-concurrent-requests", 0, "Maximum number of concurrent requests allowed on the server. When the tier2 service hits this limit, it will set itself as 'Not Ready' until requests are processed. Default 0 (no limit)")

			// all substreams
			registerCommonSubstreamsFlags(cmd)
			return nil
		},

		FactoryFunc: func(runtime *launcher.Runtime) (launcher.App, error) {
			rawServiceDiscoveryURL := viper.GetString("substreams-tier2-discovery-service-url")
			grpcListenAddr := viper.GetString("substreams-tier2-grpc-listen-addr")

			maximumConcurrentRequests := viper.GetUint64("substreams-tier2-max-concurrent-requests")

			tracing := os.Getenv("SUBSTREAMS_TRACING") == "modules_exec"

			var serviceDiscoveryURL *url.URL
			if rawServiceDiscoveryURL != "" {
				var err error
				svcURL, err := url.Parse(rawServiceDiscoveryURL)
				if err != nil {
					return nil, fmt.Errorf("unable to parse discovery service url: %w", err)
				}
				err = discoveryservice.Bootstrap(svcURL)
				if err != nil {
					return nil, fmt.Errorf("unable to bootstrap discovery service: %w", err)
				}
				serviceDiscoveryURL = svcURL
			}

			var wasmExtensions wasm.WASMExtensioner
			if chain.RegisterSubstreamsExtensions != nil {
				exts, err := chain.RegisterSubstreamsExtensions()
				if err != nil {
					return nil, fmt.Errorf("substreams extensions: %w", err)
				}
				wasmExtensions = exts
			}

			return app.NewTier2(appLogger,
				&app.Tier2Config{
					Tracing: tracing,

					GRPCListenAddr:      grpcListenAddr,
					ServiceDiscoveryURL: serviceDiscoveryURL,
					WASMExtensions:      wasmExtensions,

					MaximumConcurrentRequests: maximumConcurrentRequests,
				}, &app.Tier2Modules{
					CheckPendingShutDown: runtime.IsPendingShutdown,
				}), nil
		},
	})
}
