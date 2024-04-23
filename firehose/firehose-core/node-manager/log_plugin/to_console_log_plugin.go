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
	"fmt"
	"os"
	"strconv"

	"github.com/streamingfast/shutter"
)

var DebugLineLength = int64(4096)

func init() {
	if os.Getenv("DEBUG_LINE_LENGTH") != "" {
		value, err := strconv.ParseInt(os.Getenv("DEBUG_LINE_LENGTH"), 10, 64)
		if err == nil {
			DebugLineLength = value
		}
	}
}

// ToConsoleLogPlugin takes a line, and if it's not a FIRE (or DMLOG) line or
// if we are actively debugging deep mind, will print the line to the
// standard output
type ToConsoleLogPlugin struct {
	*shutter.Shutter
	debugDeepMind  bool
	skipBlankLines bool
}

func NewToConsoleLogPlugin(debugDeepMind bool) *ToConsoleLogPlugin {
	return &ToConsoleLogPlugin{
		Shutter:       shutter.New(),
		debugDeepMind: debugDeepMind,
	}
}

func (p *ToConsoleLogPlugin) SetSkipBlankLines(skip bool) {
	p.skipBlankLines = skip
}

func (p *ToConsoleLogPlugin) Launch() {}
func (p ToConsoleLogPlugin) Stop()    {}
func (p *ToConsoleLogPlugin) Name() string {
	return "ToConsoleLogPlugin"
}
func (p *ToConsoleLogPlugin) DebugDeepMind(enabled bool) {
	p.debugDeepMind = enabled
}

func (p *ToConsoleLogPlugin) LogLine(in string) {
	if in == "" && p.skipBlankLines {
		return
	}

	if p.debugDeepMind || !readerInstrumentationPrefixRegex.MatchString(in) {
		logLineLength := int64(len(in))

		// We really want to write lines to stdout and not through our logger, it's the purpose of our plugin!
		if logLineLength > DebugLineLength {
			fmt.Printf("%s ... bytes: %d\n", in[:DebugLineLength], (logLineLength - DebugLineLength))
		} else {
			fmt.Println(in)
		}
	}
}
