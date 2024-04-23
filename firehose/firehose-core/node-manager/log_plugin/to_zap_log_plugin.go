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

	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

var NoDisplay = zapcore.Level(zap.FatalLevel + 10)

type ToZapLogPluginOption interface {
	apply(p *ToZapLogPlugin)
}

type toZapLogPluginOptionFunc func(p *ToZapLogPlugin)

func (s toZapLogPluginOptionFunc) apply(p *ToZapLogPlugin) {
	s(p)
}

// ToZapLogPluginLogLevel is the option that defines which function to use to extract the log level
// from the line.
//
// The received function will be invoked with the actual line to log. The function should then return
// the log level value to use for this line. If the return value is the special value `NoDisplay` constant
// (which corresponds to log level `15` which does not exist within zap), the line is actually discarded
// completely and not logged to the logger.
func ToZapLogPluginLogLevel(extractLevel func(in string) zapcore.Level) ToZapLogPluginOption {
	return toZapLogPluginOptionFunc(func(p *ToZapLogPlugin) {
		p.levelExtractor = extractLevel
	})
}

// ToZapLogPluginTransformer is the option that defines which function to use to transform the line before
// being logged to the logger.
//
// The received function will be invoked with the actual line to log **after** the level have been determined.
// The function should then return the transformed line. If the return line is the empty string, it is discarded
// completely.
func ToZapLogPluginTransformer(transformer func(in string) string) ToZapLogPluginOption {
	return toZapLogPluginOptionFunc(func(p *ToZapLogPlugin) {
		p.lineTransformer = transformer
	})
}

// ToZapLogPlugin takes a line, and if it's not a FIRE (or DMLOG) line or
// if we are actively debugging deep mind, will print the line to received
// logger instance.
type ToZapLogPlugin struct {
	*shutter.Shutter

	logger        *zap.Logger
	debugDeepMind bool

	levelExtractor  func(in string) zapcore.Level
	lineTransformer func(in string) string
}

func NewToZapLogPlugin(debugDeepMind bool, logger *zap.Logger, options ...ToZapLogPluginOption) *ToZapLogPlugin {
	plugin := &ToZapLogPlugin{
		Shutter:       shutter.New(),
		debugDeepMind: debugDeepMind,
		logger:        logger,
	}

	for _, opt := range options {
		opt.apply(plugin)
	}

	return plugin
}

func (p *ToZapLogPlugin) Launch() {}
func (p ToZapLogPlugin) Stop()    {}

func (p *ToZapLogPlugin) Name() string {
	return "ToZapLogPlugin"
}

func (p *ToZapLogPlugin) DebugDeepMind(enabled bool) {
	p.debugDeepMind = enabled
}

//func (p *ToZapLogPlugin) Close(_ error) {
//}

func (p *ToZapLogPlugin) LogLine(in string) {
	if readerInstrumentationPrefixRegex.MatchString(in) {
		if p.debugDeepMind {
			// Needs to be an info since often used in production where debug level is not enabled by default
			p.logger.Info(in)
		}

		return
	}

	level := zap.DebugLevel
	if p.levelExtractor != nil {
		level = p.levelExtractor(in)
		if level == NoDisplay {
			// This is ignored, nothing else to do here ...
			return
		}
	}

	if p.lineTransformer != nil {
		in = p.lineTransformer(in)
		if in == "" {
			// This is ignored, nothing else to do here ...
			return
		}
	}

	p.logger.Check(level, in).Write()
}
