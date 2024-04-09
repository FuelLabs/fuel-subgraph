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

package launcher

import (
	"path/filepath"

	"github.com/streamingfast/logging"
	"go.uber.org/zap"
)

type LoggingOptions struct {
	WorkingDir    string // the folder where the data will be stored, in our case will be used to store the logger
	Verbosity     int    // verbosity level
	LogFormat     string // specifies the log format
	LogToFile     bool   // specifies if we should store the logs on disk
	LogListenAddr string // address that listens to change the logs
	LogToStderr   bool   // determines if the standard console logger should log to Stderr (defaults is to log in Stdout)
}

func SetupLogger(rootLogger *zap.Logger, opts *LoggingOptions) {
	options := []logging.InstantiateOption{
		logging.WithLogLevelSwitcherServerAutoStart(),
		logging.WithDefaultSpec(defaultSpecForVerbosity(opts.Verbosity)...),
		logging.WithConsoleToStdout(),
	}

	if opts.LogToStderr {
		options = append(options, logging.WithConsoleToStderr())
	}

	if opts.LogListenAddr != "" {
		options = append(options, logging.WithLogLevelSwitcherServerListeningAddress(opts.LogListenAddr))
	}

	if opts.LogFormat == "stackdriver" || opts.LogFormat == "json" {
		options = append(options, logging.WithProductionLogger())
	}

	if opts.LogToFile {
		options = append(options, logging.WithOutputToFile(filepath.Join(opts.WorkingDir, "app.log.json")))
	}

	logging.InstantiateLoggers(options...)

	// Hijack standard Golang `log` and redirect it to our common logger
	zap.RedirectStdLogAt(rootLogger, zap.DebugLevel)
}

func defaultSpecForVerbosity(verbosity int) []string {
	switch verbosity {
	case 0:
		return nil

	case 1:
		return []string{"*=info"}

	default:
		return []string{"*=debug"}
	}
}
