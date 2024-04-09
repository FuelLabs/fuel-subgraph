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

package node_manager

import (
	"time"

	logplugin "github.com/streamingfast/firehose-core/node-manager/log_plugin"
)

type StartOption string

var EnableDebugDeepmindOption = StartOption("enable-debug-firehose-logs")
var DisableDebugDeepmindOption = StartOption("disable-debug-firehose-logs")

type ShutterInterface interface {
	Shutdown(error)
	OnTerminating(func(error))
	OnTerminated(func(error))
	IsTerminated() bool
	IsTerminating() bool
	Terminated() <-chan struct{}
}

type ChainSuperviser interface {
	ShutterInterface

	GetCommand() string
	GetName() string
	ServerID() (string, error)

	RegisterLogPlugin(plugin logplugin.LogPlugin)
	Start(options ...StartOption) error
	Stop() error

	IsRunning() bool
	Stopped() <-chan struct{}

	LastExitCode() int
	LastLogLines() []string
	LastSeenBlockNum() uint64
}

type MonitorableChainSuperviser interface {
	Monitor()
}

type ProducerChainSuperviser interface {
	IsProducing() (bool, error)
	IsActiveProducer() bool

	ResumeProduction() error
	PauseProduction() error

	WaitUntilEndOfNextProductionRound(timeout time.Duration) error
}

type ProductionState int

const (
	StatePre       ProductionState = iota // Just before we produce, don't restart
	StateProducing                        // We're producing right now
	StatePost                             // Right after production
	StateStale                            // We haven't produced for 12 minutes
)

func (s ProductionState) String() string {
	switch s {
	case StatePre:
		return "pre"
	case StateProducing:
		return "producing"
	case StatePost:
		return "post"
	case StateStale:
		return "stale"
	default:
		return "unknown"
	}
}

type ProductionEvent int

const (
	EventProduced ProductionEvent = iota
	EventReceived
)
