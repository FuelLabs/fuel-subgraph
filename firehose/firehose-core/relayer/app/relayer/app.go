// Copyright 2019 dfuse Platform Inc.
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

package relayer

import (
	"context"
	"fmt"
	"time"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/dmetrics"
	"github.com/streamingfast/dstore"
	"github.com/streamingfast/firehose-core/relayer"
	"github.com/streamingfast/firehose-core/relayer/metrics"
	"github.com/streamingfast/shutter"
	"go.uber.org/zap"
	pbhealth "google.golang.org/grpc/health/grpc_health_v1"
)

var RelayerStartAborted = fmt.Errorf("getting start block aborted by relayer application terminating signal")

type Config struct {
	SourcesAddr        []string
	GRPCListenAddr     string
	SourceRequestBurst int
	MaxSourceLatency   time.Duration
	OneBlocksURL       string
}

func (c *Config) ZapFields() []zap.Field {
	return []zap.Field{
		zap.Strings("sources_addr", c.SourcesAddr),
		zap.String("grpc_listen_addr", c.GRPCListenAddr),
		zap.Int("source_request_burst", c.SourceRequestBurst),
		zap.Duration("max_source_latency", c.MaxSourceLatency),
		zap.String("one_blocks_url", c.OneBlocksURL),
	}
}

type App struct {
	*shutter.Shutter
	config *Config

	relayer *relayer.Relayer
}

func New(config *Config) *App {
	return &App{
		Shutter: shutter.New(),
		config:  config,
	}
}

func (a *App) Run() error {
	dmetrics.Register(metrics.MetricSet)

	oneBlocksStore, err := dstore.NewDBinStore(a.config.OneBlocksURL)
	if err != nil {
		return fmt.Errorf("getting block store: %w", err)
	}

	liveSourceFactory := bstream.SourceFactory(func(h bstream.Handler) bstream.Source {
		return relayer.NewMultiplexedSource(
			h,
			a.config.SourcesAddr,
			a.config.MaxSourceLatency,
			a.config.SourceRequestBurst,
		)
	})
	oneBlocksSourceFactory := bstream.SourceFromNumFactoryWithSkipFunc(func(num uint64, h bstream.Handler, skipFunc func(idSuffix string) bool) bstream.Source {
		src, err := bstream.NewOneBlocksSource(num, oneBlocksStore, h, bstream.OneBlocksSourceWithSkipperFunc(skipFunc))
		if err != nil {
			return nil
		}
		return src
	})

	zlog.Info("starting relayer", a.config.ZapFields()...)
	a.relayer = relayer.NewRelayer(
		liveSourceFactory,
		oneBlocksSourceFactory,
		a.config.GRPCListenAddr,
	)

	a.OnTerminating(a.relayer.Shutdown)
	a.relayer.OnTerminated(a.Shutdown)

	a.relayer.Run()
	return nil
}

var emptyHealthCheckRequest = &pbhealth.HealthCheckRequest{}

func (a *App) IsReady() bool {
	if a.relayer == nil {
		return false
	}

	resp, err := a.relayer.Check(context.Background(), emptyHealthCheckRequest)
	if err != nil {
		zlog.Info("readiness check failed", zap.Error(err))
		return false
	}

	return resp.Status == pbhealth.HealthCheckResponse_SERVING
}
