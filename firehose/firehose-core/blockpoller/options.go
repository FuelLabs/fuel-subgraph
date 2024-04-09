package blockpoller

import "go.uber.org/zap"

type Option func(*BlockPoller)

func WithBlockFetchRetryCount(v uint64) Option {
	return func(p *BlockPoller) {
		p.fetchBlockRetryCount = v
	}
}

func WithStoringState(stateStorePath string) Option {
	return func(p *BlockPoller) {
		p.stateStorePath = stateStorePath
	}
}

// IgnoreCursor ensures the poller will ignore the cursor and start from the startBlockNum
// the cursor will still be saved as the poller progresses
func IgnoreCursor() Option {
	return func(p *BlockPoller) {
		p.ignoreCursor = true
	}
}

func WithLogger(logger *zap.Logger) Option {
	return func(p *BlockPoller) {
		p.logger = logger
	}
}
