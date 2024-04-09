package rate

import (
	"context"
	"fmt"
	"time"
)

type Limiter interface {
	Take(ctx context.Context, id string, method string) (allow bool)
	Return()
	String() string
}

type token bool

type leakyBucketLimiter struct {
	tokens chan token

	dripInterval time.Duration
}

func NewLeakyBucketLimiter(size int, dripInterval time.Duration) Limiter {
	tks := make(chan token, size)
	for i := 0; i < size; i++ {
		tks <- token(true)
	}

	go func() {
		for {
			select {
			case <-time.After(dripInterval):
				select {
				case tks <- token(true):
					//
				default:
					//
				}
			}
		}
	}()

	return &leakyBucketLimiter{
		tokens:       tks,
		dripInterval: dripInterval,
	}
}

func (l *leakyBucketLimiter) Take(ctx context.Context, id string, method string) (allow bool) {
	select {
	case <-l.tokens:
		return true
	case <-ctx.Done():
		return false
	default:
		return false
	}
}

func (l *leakyBucketLimiter) Return() {
	select {
	case l.tokens <- token(true):
		//
	default:
		//
	}
}

func (l *leakyBucketLimiter) String() string {
	return fmt.Sprintf("leaky-bucket-limiter(len=%d, cap=%d, drip-interval=%s)", len(l.tokens), cap(l.tokens), l.dripInterval)
}
