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
	"strings"
	"time"

	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/bstream/blockstream"
	"github.com/streamingfast/bstream/forkable"
	"github.com/streamingfast/bstream/hub"
	dgrpcfactory "github.com/streamingfast/dgrpc/server/factory"
	"github.com/streamingfast/firehose-core/relayer/metrics"
	"github.com/streamingfast/shutter"
	pbhealth "google.golang.org/grpc/health/grpc_health_v1"
)

const (
	getHeadInfoTimeout = 10 * time.Second
)

type Relayer struct {
	*shutter.Shutter

	grpcListenAddr         string
	liveSourceFactory      bstream.SourceFactory
	oneBlocksSourceFactory bstream.SourceFromNumFactoryWithSkipFunc

	hub *hub.ForkableHub

	ready bool

	blockStreamServer *hub.BlockstreamServer
}

func NewRelayer(
	liveSourceFactory bstream.SourceFactory,
	oneBlocksSourceFactory bstream.SourceFromNumFactoryWithSkipFunc,
	grpcListenAddr string,
) *Relayer {
	r := &Relayer{
		Shutter:                shutter.New(),
		grpcListenAddr:         grpcListenAddr,
		liveSourceFactory:      liveSourceFactory,
		oneBlocksSourceFactory: oneBlocksSourceFactory,
	}

	gs := dgrpcfactory.ServerFromOptions()
	pbhealth.RegisterHealthServer(gs.ServiceRegistrar(), r)

	forkableHub := hub.NewForkableHub(
		r.liveSourceFactory,
		r.oneBlocksSourceFactory,
		10,
		forkable.EnsureAllBlocksTriggerLongestChain(), // send every forked block too
		forkable.WithFilters(bstream.StepNew),
		forkable.WithFailOnUnlinkableBlocks(20, time.Minute),
	)
	r.hub = forkableHub
	gs.OnTerminated(r.Shutdown)
	r.blockStreamServer = r.hub.NewBlockstreamServer(gs)
	return r

}

func NewMultiplexedSource(handler bstream.Handler, sourceAddresses []string, maxSourceLatency time.Duration, sourceRequestBurst int) bstream.Source {
	ctx := context.Background()

	var sourceFactories []bstream.SourceFactory
	for _, u := range sourceAddresses {

		url := u // https://github.com/golang/go/wiki/CommonMistakes (url is given to the blockstream newSource)
		sourceName := urlToLoggerName(url)
		logger := zlog.Named("src").Named(sourceName)
		sf := func(subHandler bstream.Handler) bstream.Source {

			gate := bstream.NewRealtimeGate(maxSourceLatency, subHandler, bstream.GateOptionWithLogger(logger))
			var upstreamHandler bstream.Handler
			upstreamHandler = bstream.HandlerFunc(func(blk *pbbstream.Block, obj interface{}) error {
				return gate.ProcessBlock(blk, &namedObj{
					Obj:  obj,
					Name: sourceName,
				})
			})

			src := blockstream.NewSource(ctx, url, int64(sourceRequestBurst), upstreamHandler, blockstream.WithLogger(logger), blockstream.WithRequester("relayer"))
			return src
		}
		sourceFactories = append(sourceFactories, sf)
	}

	return bstream.NewMultiplexedSource(sourceFactories, handler, bstream.MultiplexedSourceWithLogger(zlog))
}

func urlToLoggerName(url string) string {
	return strings.TrimPrefix(strings.TrimPrefix(url, "dns:///"), ":")
}

func pollMetrics(fh *hub.ForkableHub) {
	for {
		time.Sleep(time.Second * 2)
		headNum, _, headTime, _, err := fh.HeadInfo()
		if err != nil {
			zlog.Info("cannot get head info yet")
			continue
		}
		metrics.HeadBlockTimeDrift.SetBlockTime(headTime)
		metrics.HeadBlockNumber.SetUint64(headNum)
	}
}

func (r *Relayer) Run() {
	go r.hub.Run()
	zlog.Info("waiting for hub to be ready...")
	<-r.hub.Ready
	go pollMetrics(r.hub)

	r.OnTerminating(func(e error) {
		zlog.Info("closing block stream server")
		r.blockStreamServer.Close()
	})

	r.blockStreamServer.Launch(r.grpcListenAddr)

	zlog.Info("relayer started")
	r.ready = true
	metrics.AppReadiness.SetReady()

	<-r.hub.Terminating()
	r.Shutdown(r.hub.Err())
}

type namedObj struct {
	Name string
	Obj  interface{}
}
