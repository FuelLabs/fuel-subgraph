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
	"github.com/streamingfast/shutter"
)

// KeepLastLinesLogPlugin takes a line and keep the last N lines as requested by the caller.
type KeepLastLinesLogPlugin struct {
	*shutter.Shutter
	lastLines            *lineRingBuffer
	includeDeepMindLines bool
}

func NewKeepLastLinesLogPlugin(lineCount int, includeDeepMindLines bool) *KeepLastLinesLogPlugin {
	plugin := &KeepLastLinesLogPlugin{
		Shutter:              shutter.New(),
		lastLines:            &lineRingBuffer{maxCount: lineCount},
		includeDeepMindLines: includeDeepMindLines,
	}

	return plugin
}
func (p *KeepLastLinesLogPlugin) Name() string {
	return "KeepLastLinesLogPlugin"
}
func (p *KeepLastLinesLogPlugin) Launch() {}
func (p KeepLastLinesLogPlugin) Stop()    {}
func (p *KeepLastLinesLogPlugin) DebugDeepMind(enabled bool) {
	p.includeDeepMindLines = enabled
}

func (p *KeepLastLinesLogPlugin) LastLines() []string {
	return p.lastLines.lines()
}

//func (p *KeepLastLinesLogPlugin) Close(_ error) {
//}

func (p *KeepLastLinesLogPlugin) LogLine(in string) {
	if readerInstrumentationPrefixRegex.MatchString(in) && !p.includeDeepMindLines {
		// It's a deep mind log line and we don't care about it, skip
		return
	}

	p.lastLines.append(in)
}
