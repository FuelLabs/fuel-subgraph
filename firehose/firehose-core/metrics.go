package firecore

import "github.com/streamingfast/dmetrics"

func RegisterMetrics() {
	metrics.Register()
}

var metrics = dmetrics.NewSet()

var ConsoleReaderBlockReadCount = metrics.NewCounter("firecore_console_reader_block_read_count", "Number of blocks read by the console reader")
