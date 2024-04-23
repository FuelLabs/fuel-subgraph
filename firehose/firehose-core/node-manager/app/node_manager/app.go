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

package nodemanager

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"time"

	dgrpcserver "github.com/streamingfast/dgrpc/server"
	dgrpcfactory "github.com/streamingfast/dgrpc/server/factory"
	"github.com/streamingfast/dmetrics"
	nodeManager "github.com/streamingfast/firehose-core/node-manager"
	"github.com/streamingfast/firehose-core/node-manager/metrics"
	"github.com/streamingfast/firehose-core/node-manager/mindreader"
	"github.com/streamingfast/firehose-core/node-manager/operator"
	"github.com/streamingfast/shutter"
	"go.uber.org/zap"
	"google.golang.org/grpc"
)

type Config struct {
	StartupDelay time.Duration

	HTTPAddr           string // was ManagerAPIAddress
	ConnectionWatchdog bool

	GRPCAddr string
}

type Modules struct {
	Operator                     *operator.Operator
	MetricsAndReadinessManager   *nodeManager.MetricsAndReadinessManager
	LaunchConnectionWatchdogFunc func(terminating <-chan struct{})
	MindreaderPlugin             *mindreader.MindReaderPlugin
	RegisterGRPCService          func(server grpc.ServiceRegistrar) error
	StartFailureHandlerFunc      func()
}

type App struct {
	*shutter.Shutter
	config  *Config
	modules *Modules
	zlogger *zap.Logger
}

func New(config *Config, modules *Modules, zlogger *zap.Logger) *App {
	return &App{
		Shutter: shutter.New(),
		config:  config,
		modules: modules,
		zlogger: zlogger,
	}
}

func (a *App) Run() error {
	hasMindreader := a.modules.MindreaderPlugin != nil
	a.zlogger.Info("running node manager app", zap.Reflect("config", a.config), zap.Bool("mindreader", hasMindreader))

	hostname, _ := os.Hostname()
	a.zlogger.Info("retrieved hostname from os", zap.String("hostname", hostname))

	dmetrics.Register(metrics.Metricset)

	a.OnTerminating(func(err error) {
		a.modules.Operator.Shutdown(err)
		<-a.modules.Operator.Terminated()
	})

	a.modules.Operator.OnTerminated(func(err error) {
		a.zlogger.Info("chain operator terminated shutting down mindreader app")
		a.Shutdown(err)
	})

	if a.config.StartupDelay != 0 {
		time.Sleep(a.config.StartupDelay)
	}

	var httpOptions []operator.HTTPOption
	if hasMindreader {
		if err := a.startMindreader(); err != nil {
			return fmt.Errorf("unable to start mindreader: %w", err)
		}

	}

	a.zlogger.Info("launching operator")
	go a.modules.MetricsAndReadinessManager.Launch()
	go a.Shutdown(a.modules.Operator.Launch(a.config.HTTPAddr, httpOptions...))

	if a.config.ConnectionWatchdog {
		go a.modules.LaunchConnectionWatchdogFunc(a.Terminating())
	}

	return nil
}

func (a *App) IsReady() bool {
	ctx, cancel := context.WithTimeout(context.Background(), 100*time.Millisecond)
	defer cancel()

	url := fmt.Sprintf("http://%s/healthz", a.config.HTTPAddr)
	req, err := http.NewRequestWithContext(ctx, "GET", url, nil)
	if err != nil {
		a.zlogger.Warn("unable to build get health request", zap.Error(err))
		return false
	}

	client := http.DefaultClient
	res, err := client.Do(req)
	if err != nil {
		a.zlogger.Debug("unable to execute get health request", zap.Error(err))
		return false
	}

	return res.StatusCode == 200
}

func (a *App) startMindreader() error {
	a.zlogger.Info("starting mindreader gRPC server")
	gs := dgrpcfactory.ServerFromOptions(dgrpcserver.WithLogger(a.zlogger))

	if a.modules.RegisterGRPCService != nil {
		err := a.modules.RegisterGRPCService(gs.ServiceRegistrar())
		if err != nil {
			return fmt.Errorf("register extra grpc service: %w", err)
		}
	}

	gs.OnTerminated(a.Shutdown)

	// Launch is blocking and we don't want to block in this method
	go gs.Launch(a.config.GRPCAddr)

	return nil
}
