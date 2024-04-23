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
	"strings"
	"testing"

	"github.com/streamingfast/logging"
	"github.com/stretchr/testify/assert"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

func TestToZapLogPlugin(t *testing.T) {
	simpleExtractor := func(in string) zapcore.Level {
		if strings.HasPrefix(in, "error") {
			return zap.ErrorLevel
		}

		if strings.HasPrefix(in, "discard") {
			return NoDisplay
		}

		return zap.InfoLevel
	}

	simpleTransformer := func(in string) string {
		if in == "message" {
			return in
		}

		if in == "discard" {
			return ""
		}

		return strings.ToUpper(in)
	}

	toUnderscoreTransformer := func(in string) string {
		return "_"
	}

	options := func(opts ...ToZapLogPluginOption) []ToZapLogPluginOption {
		return opts
	}

	tests := []struct {
		name    string
		in      []string
		options []ToZapLogPluginOption
		out     []string
	}{
		// Plain
		{
			"plain, always debug untransformed",
			[]string{"error message"},
			nil,
			[]string{`{"level":"debug","msg":"error message"}`},
		},

		// Log Level
		{
			"with log leve, match element",
			[]string{"error message"},
			options(ToZapLogPluginLogLevel(simpleExtractor)),
			[]string{`{"level":"error","msg":"error message"}`},
		},
		{
			"with log leve, no match element",
			[]string{"warn message"},
			options(ToZapLogPluginLogLevel(simpleExtractor)),
			[]string{`{"level":"info","msg":"warn message"}`},
		},
		{
			"with log leve, no display element",
			[]string{"discard message"},
			options(ToZapLogPluginLogLevel(simpleExtractor)),
			[]string(nil),
		},

		// Transformer
		{
			"with transformer, same",
			[]string{"message"},
			options(ToZapLogPluginTransformer(simpleTransformer)),
			[]string{`{"level":"debug","msg":"message"}`},
		},
		{
			"with transformer, to upper",
			[]string{"any"},
			options(ToZapLogPluginTransformer(simpleTransformer)),
			[]string{`{"level":"debug","msg":"ANY"}`},
		},
		{
			"with transformer, discard",
			[]string{"discard"},
			options(ToZapLogPluginTransformer(simpleTransformer)),
			[]string(nil),
		},

		// Log Level & Transformer
		{
			"with transformer & log leve, transform don't affect log level",
			[]string{"error message"},
			options(ToZapLogPluginTransformer(toUnderscoreTransformer), ToZapLogPluginLogLevel(simpleExtractor)),
			[]string{`{"level":"error","msg":"_"}`},
		},
		{
			"with transformer & log leve, transform don't affect log level, any option order",
			[]string{"error message"},
			options(ToZapLogPluginLogLevel(simpleExtractor), ToZapLogPluginTransformer(toUnderscoreTransformer)),
			[]string{`{"level":"error","msg":"_"}`},
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			testLogger := logging.NewTestLogger(t)

			plugin := NewToZapLogPlugin(false, testLogger.Instance(), test.options...)
			for _, in := range test.in {
				plugin.LogLine(in)
			}

			assert.Equal(t, test.out, testLogger.RecordedLines(t))
		})
	}
}
