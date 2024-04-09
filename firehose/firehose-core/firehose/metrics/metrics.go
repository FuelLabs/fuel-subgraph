package metrics

import (
	"github.com/streamingfast/dmetrics"
)

var Metricset = dmetrics.NewSet()

var AppReadiness = Metricset.NewAppReadiness("firehose")
var ActiveRequests = Metricset.NewGauge("firehose_active_requests", "Number of active requests")
var RequestCounter = Metricset.NewCounter("firehose_requests_counter", "Request count")

var ActiveSubstreams = Metricset.NewGauge("firehose_active_substreams", "Number of active substreams requests")
var SubstreamsCounter = Metricset.NewCounter("firehose_substreams_counter", "Substreams requests count")

// var CurrentListeners = Metricset.NewGaugeVec("current_listeners", []string{"req_type"}, "...")
// var TimedOutPushingTrxCount = Metricset.NewCounterVec("something", []string{"guarantee"}, "Number of requests for push_transaction timed out while submitting")
