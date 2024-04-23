package blockpoller

import (
	"context"
	"fmt"
	"testing"
	"time"

	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/streamingfast/derr"
	"github.com/streamingfast/logging"
	"github.com/stretchr/testify/assert"
	"github.com/test-go/testify/require"
	"go.uber.org/zap/zapcore"
)

var logger, tracer = logging.PackageLogger("forkhandler", "github.com/streamingfast/firehose-core/forkhandler.test")

func init() {
	logging.InstantiateLoggers(logging.WithDefaultLevel(zapcore.DebugLevel))
}

var TestErrCompleteDone = fmt.Errorf("test complete done")

type TestBlock struct {
	expect *pbbstream.Block
	send   *pbbstream.Block
}

var _ BlockFetcher = &TestBlockFetcher{}

type TestBlockFetcher struct {
	t         *testing.T
	blocks    []*TestBlock
	idx       uint64
	completed bool
}

func newTestBlockFetcher(t *testing.T, blocks []*TestBlock) *TestBlockFetcher {
	return &TestBlockFetcher{
		t:      t,
		blocks: blocks,
	}
}

func (b *TestBlockFetcher) PollingInterval() time.Duration {
	return 0
}

func (b *TestBlockFetcher) IsBlockAvailable(requestedSlot uint64) bool {
	return true
}

func (b *TestBlockFetcher) Fetch(_ context.Context, blkNum uint64) (*pbbstream.Block, bool, error) {
	if len(b.blocks) == 0 {
		assert.Fail(b.t, fmt.Sprintf("should not have fetched block %d", blkNum))
	}

	if b.idx >= uint64(len(b.blocks)) {
		return nil, false, derr.NewFatalError(TestErrCompleteDone)
	}

	if blkNum != b.blocks[b.idx].expect.Number {
		assert.Fail(b.t, fmt.Sprintf("expected to fetch block %d, got %d", b.blocks[b.idx].expect.Number, blkNum))
	}

	blkToSend := b.blocks[b.idx].send
	b.idx++
	return blkToSend, false, nil
}

func (b *TestBlockFetcher) check(t *testing.T) {
	t.Helper()
	require.Equal(b.t, uint64(len(b.blocks)), b.idx, "we should have fetched all %d blocks, only fired %d blocks", len(b.blocks), b.idx)
}

var _ BlockHandler = &TestBlockFinalizer{}

type TestBlockFinalizer struct {
	t          *testing.T
	fireBlocks []*pbbstream.Block
	idx        uint64
}

func newTestBlockFinalizer(t *testing.T, fireBlocks []*pbbstream.Block) *TestBlockFinalizer {
	return &TestBlockFinalizer{
		t:          t,
		fireBlocks: fireBlocks,
	}
}

func (t *TestBlockFinalizer) Init() {
	//TODO implement me
	panic("implement me")
}

func (t *TestBlockFinalizer) Handle(blk *pbbstream.Block) error {
	if len(t.fireBlocks) == 0 {
		assert.Fail(t.t, fmt.Sprintf("should not have fired block %s", blk.AsRef()))
	}

	if t.idx >= uint64(len(t.fireBlocks)) {
		return TestErrCompleteDone
	}

	if blk.Number != t.fireBlocks[t.idx].Number {
		assert.Fail(t.t, fmt.Sprintf("expected to fetch block %d, got %d", t.fireBlocks[t.idx].Number, blk.Number))
	}
	t.idx++
	return nil
}

func (b *TestBlockFinalizer) check(t *testing.T) {
	t.Helper()
	require.Equal(b.t, uint64(len(b.fireBlocks)), b.idx, "we should have fired all %d blocks, only fired %d blocks", len(b.fireBlocks), b.idx)
}

var _ BlockHandler = &TestNoopBlockFinalizer{}

type TestNoopBlockFinalizer struct{}

func (t *TestNoopBlockFinalizer) Init()                             {}
func (t *TestNoopBlockFinalizer) Handle(blk *pbbstream.Block) error { return nil }
