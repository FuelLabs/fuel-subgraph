package superviser

import (
	logplugin "github.com/streamingfast/firehose-core/node-manager/log_plugin"
)

// This file configures a logging reader that transforms log lines received from the blockchain process running
// and then logs them inside the Firehose stack logging system.
//
// A default implementation uses a regex to identify the level of the line and turn it into our internal level value.
//
// You should override the `GetLogLevelFunc` above to determine the log level for your speficic chain
func NewNodeLogPlugin(debugFirehose bool) logplugin.LogPlugin {
	return logplugin.NewToConsoleLogPlugin(debugFirehose)
}
