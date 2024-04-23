package metrics

import "github.com/streamingfast/dmetrics"

var MetricSet = dmetrics.NewSet()

var HeadBlockTimeDrift = MetricSet.NewHeadTimeDrift("block-indexer")
var HeadBlockNumber = MetricSet.NewHeadBlockNumber("block-indexer")
var AppReadiness = MetricSet.NewAppReadiness("block-indexer")
