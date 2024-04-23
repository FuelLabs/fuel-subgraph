package node_manager

import (
	"time"

	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"

	"github.com/streamingfast/dmetrics"
	"go.uber.org/atomic"
	"go.uber.org/zap"
)

type Readiness interface {
	IsReady() bool
}

type MetricsAndReadinessManager struct {
	headBlockChan      chan *pbbstream.Block
	headBlockTimeDrift *dmetrics.HeadTimeDrift
	headBlockNumber    *dmetrics.HeadBlockNum
	appReadiness       *dmetrics.AppReadiness
	readinessProbe     *atomic.Bool

	// ReadinessMaxLatency is the max delta between head block time and
	// now before /healthz starts returning success
	readinessMaxLatency time.Duration

	logger *zap.Logger
}

func NewMetricsAndReadinessManager(headBlockTimeDrift *dmetrics.HeadTimeDrift, headBlockNumber *dmetrics.HeadBlockNum, appReadiness *dmetrics.AppReadiness, readinessMaxLatency time.Duration) *MetricsAndReadinessManager {
	return &MetricsAndReadinessManager{
		headBlockChan:       make(chan *pbbstream.Block, 1), // just for non-blocking, saving a few nanoseconds here
		readinessProbe:      atomic.NewBool(false),
		appReadiness:        appReadiness,
		headBlockTimeDrift:  headBlockTimeDrift,
		headBlockNumber:     headBlockNumber,
		readinessMaxLatency: readinessMaxLatency,
	}
}

func (m *MetricsAndReadinessManager) setReadinessProbeOn() {
	m.readinessProbe.CAS(false, true)
	m.appReadiness.SetReady()
}

func (m *MetricsAndReadinessManager) setReadinessProbeOff() {
	m.readinessProbe.CAS(true, false)
	m.appReadiness.SetNotReady()
}

func (m *MetricsAndReadinessManager) IsReady() bool {
	return m.readinessProbe.Load()
}

func (m *MetricsAndReadinessManager) Launch() {
	for {
		var lastSeenBlock *pbbstream.Block
		select {
		case block := <-m.headBlockChan:
			lastSeenBlock = block
		case <-time.After(time.Second):
		}

		if lastSeenBlock == nil {
			continue
		}

		// metrics
		if m.headBlockNumber != nil {
			m.headBlockNumber.SetUint64(lastSeenBlock.Number)
		}

		if lastSeenBlock.Time().IsZero() { // never act upon zero timestamps
			continue
		}
		if m.headBlockTimeDrift != nil {
			m.headBlockTimeDrift.SetBlockTime(lastSeenBlock.Time())
		}

		// readiness
		if m.readinessMaxLatency == 0 || time.Since(lastSeenBlock.Time()) < m.readinessMaxLatency {
			m.setReadinessProbeOn()
		} else {
			m.setReadinessProbeOff()
		}
	}
}

func (m *MetricsAndReadinessManager) UpdateHeadBlock(block *pbbstream.Block) error {
	m.headBlockChan <- block
	return nil
}
