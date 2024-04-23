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

package logplugin

import (
	"regexp"

	"github.com/streamingfast/bstream/blockstream"
)

var readerInstrumentationPrefixRegex = regexp.MustCompile("^(DMLOG|FIRE) ")

type LogPlugin interface {
	Name() string
	Launch()
	LogLine(in string)
	//Close(err error)
	Shutdown(err error)
	IsTerminating() bool
	Stop()
}

type Shutter interface {
	Terminated() <-chan struct{}
	OnTerminating(f func(error))
	OnTerminated(f func(error))
	IsTerminating() bool
	Shutdown(err error)
}

type BlockStreamer interface {
	Run(blockServer *blockstream.Server)
}

type LogPluginFunc func(line string)

func (f LogPluginFunc) Launch()             {}
func (f LogPluginFunc) LogLine(line string) { f(line) }
func (f LogPluginFunc) Name() string        { return "log plug func" }
func (f LogPluginFunc) Stop()               {}
func (f LogPluginFunc) Shutdown(_ error)    {}
func (f LogPluginFunc) Terminated() <-chan struct{} {
	ch := make(chan struct{})
	close(ch)
	return ch
}

func (f LogPluginFunc) IsTerminating() bool {
	return false
}

func (f LogPluginFunc) OnTerminating(_ func(error)) {
}

func (f LogPluginFunc) OnTerminated(_ func(error)) {
}
