package firehose

import (
	"context"
	"sync"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/spf13/cobra"
	"github.com/streamingfast/bstream"
	firecore "github.com/streamingfast/firehose-core"
	"github.com/streamingfast/logging"
	pbfirehose "github.com/streamingfast/pbgo/sf/firehose/v2"
	"go.uber.org/zap"
)

var status = prometheus.NewGaugeVec(prometheus.GaugeOpts{Name: "firehose_healthcheck_status", Help: "Either 1 for successful firehose request, or 0 for failure"}, []string{"endpoint"})
var propagationDelay = prometheus.NewGaugeVec(prometheus.GaugeOpts{Name: "firehose_healthcheck_block_delay", Help: "Delay between block time and propagation to firehose clients"}, []string{"endpoint"})

var lastBlockLock sync.Mutex
var lastBlockReceived time.Time
var driftSec = prometheus.NewGaugeVec(prometheus.GaugeOpts{Name: "firehose_healthcheck_drift", Help: "Time since the most recent block received (seconds)"}, []string{"endpoint"})

// You should add your custom 'transforms' flags to this command in your init(), then parse them in transformsSetter
func NewToolsFirehosePrometheusExporterCmd[B firecore.Block](chain *firecore.Chain[B], zlog *zap.Logger, tracer logging.Tracer) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "firehose-prometheus-exporter <endpoint:port>",
		Short: "stream blocks near the chain HEAD and report to prometheus",
		Args:  cobra.ExactArgs(1),
		RunE:  runPrometheusExporterE(chain, zlog, tracer),
	}

	addFirehoseStreamClientFlagsToSet(cmd.Flags(), chain)

	return cmd
}

func runPrometheusExporterE[B firecore.Block](chain *firecore.Chain[B], zlog *zap.Logger, tracer logging.Tracer) func(cmd *cobra.Command, args []string) error {
	return func(cmd *cobra.Command, args []string) error {
		ctx := context.Background()

		endpoint := args[0]
		start := int64(-1)
		stop := uint64(0)

		firehoseClient, connClose, requestInfo, err := getFirehoseStreamClientFromCmd(cmd, zlog, endpoint, chain)
		if err != nil {
			return err
		}
		defer connClose()

		request := &pbfirehose.Request{
			StartBlockNum:   start,
			StopBlockNum:    stop,
			Transforms:      requestInfo.Transforms,
			FinalBlocksOnly: true,
			Cursor:          requestInfo.Cursor,
		}

		prometheus.MustRegister(status)
		prometheus.MustRegister(propagationDelay)
		prometheus.MustRegister(driftSec)

		// update the drift based on last time
		go func() {
			for {
				time.Sleep(500 * time.Millisecond)
				lastBlockLock.Lock()
				driftSec.With(prometheus.Labels{"endpoint": endpoint}).Set(time.Since(lastBlockReceived).Seconds())
				lastBlockLock.Unlock()
			}
		}()

		var sleepTime time.Duration
		for {
			time.Sleep(sleepTime)
			sleepTime = time.Second * 3
			stream, err := firehoseClient.Blocks(ctx, request, requestInfo.GRPCCallOpts...)
			if err != nil {
				zlog.Error("connecting", zap.Error(err))
				markFailure(endpoint)
				continue
			}

			zlog.Info("connected")

			for {
				response, err := stream.Recv()
				if err != nil {
					zlog.Error("got error from stream", zap.Error(err))
					markFailure(endpoint)
					break
				}

				if cursor, err := bstream.CursorFromOpaque(response.Cursor); err == nil {
					zlog.Info("Got block", zap.String("block", cursor.Block.ID()), zap.Uint64("block_num", cursor.Block.Num()))

					lastBlockLock.Lock()
					lastBlockReceived = time.Now()
					lastBlockLock.Unlock()
					markSuccess(endpoint)
				}
			}

		}

		//	serve := http.Server{Handler: handler, Addr: addr}
		//	if err := serve.ListenAndServe(); err != nil {
		//		zlog.Error("can't listen on the metrics endpoint", zap.Error(err))
		//		return err
		//	}
		//	return nil
	}
}

func markSuccess(endpoint string) {
	status.With(prometheus.Labels{"endpoint": endpoint}).Set(1)
}

func markFailure(endpoint string) {
	status.With(prometheus.Labels{"endpoint": endpoint}).Set(0)
}
