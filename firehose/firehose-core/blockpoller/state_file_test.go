package blockpoller

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/bstream/forkable"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"go.uber.org/zap"
)

func TestFireBlockFinalizer_state(t *testing.T) {
	dirName, err := os.MkdirTemp("", "fblk")
	require.NoError(t, err)
	defer os.Remove(dirName)

	fk := forkable.NewForkDB()
	// simulating a flow where the lib libmoves
	fk.SetLIB(bstream.NewBlockRef("100a", 100), 100)
	fk.AddLink(bstream.NewBlockRef("101a", 101), "100a", &block{Block: blk("101a", "100a", 100)})
	fk.AddLink(bstream.NewBlockRef("102a", 102), "101a", &block{Block: blk("102a", "101a", 100)})
	fk.AddLink(bstream.NewBlockRef("103a", 103), "102a", &block{Block: blk("103a", "102a", 100)})
	fk.SetLIB(bstream.NewBlockRef("103a", 103), 101)
	fk.AddLink(bstream.NewBlockRef("104b", 104), "103b", &block{Block: blk("104b", "103b", 101)})
	fk.AddLink(bstream.NewBlockRef("103a", 103), "102a", &block{Block: blk("103a", "102a", 101)})
	fk.AddLink(bstream.NewBlockRef("104a", 104), "103a", &block{Block: blk("104a", "103a", 101)})
	fk.AddLink(bstream.NewBlockRef("105b", 105), "104b", &block{Block: blk("105b", "104b", 101)})
	fk.AddLink(bstream.NewBlockRef("103b", 103), "102b", &block{Block: blk("103b", "102b", 101)})
	fk.AddLink(bstream.NewBlockRef("102b", 102), "101a", &block{Block: blk("102b", "101a", 101)})
	fk.AddLink(bstream.NewBlockRef("106a", 106), "105a", &block{Block: blk("106a", "105a", 101)})
	fk.AddLink(bstream.NewBlockRef("105a", 105), "104a", &block{Block: blk("105a", "104a", 101)})
	expectedBlocks, reachedLib := fk.CompleteSegment(blk("105a", "104a", 101).AsRef())
	// simulate firing the blocks
	for _, blk := range expectedBlocks {
		blk.Object.(*block).fired = true
	}
	assert.True(t, reachedLib)
	require.Equal(t, 5, len(expectedBlocks))

	expectedStateFileCnt := `{"Lib":{"id":"101a","num":101},"LastFiredBlock":{"id":"105a","num":105,"previous_ref_id":"104a"},"Blocks":[{"id":"101a","num":101,"previous_ref_id":"100a"},{"id":"102a","num":102,"previous_ref_id":"101a"},{"id":"103a","num":103,"previous_ref_id":"102a"},{"id":"104a","num":104,"previous_ref_id":"103a"},{"id":"105a","num":105,"previous_ref_id":"104a"}]}`

	poller := &BlockPoller{
		stateStorePath: dirName,
		forkDB:         fk,
		logger:         zap.NewNop(),
	}
	require.NoError(t, poller.saveState(expectedBlocks))

	filePath := filepath.Join(dirName, "cursor.json")
	cnt, err := os.ReadFile(filePath)
	require.NoError(t, err)
	assert.Equal(t, expectedStateFileCnt, string(cnt))

	forkDB, startBlock, err := initState(bstream.NewBlockRef("60a", 60), dirName, false, zap.NewNop())
	require.NoError(t, err)

	blocks, reachedLib := forkDB.CompleteSegment(bstream.NewBlockRef("105a", 105))
	assert.True(t, reachedLib)
	assertForkableBlocks(t, expectedBlocks, blocks)
	assert.Equal(t, bstream.NewBlockRef("105a", 105), startBlock)
	assert.Equal(t, "101a", forkDB.LIBID())
	assert.Equal(t, uint64(101), forkDB.LIBNum())
}

func TestFireBlockFinalizer_noSstate(t *testing.T) {
	dirName, err := os.MkdirTemp("", "fblk")
	require.NoError(t, err)
	defer os.Remove(dirName)

	forkDB, startBlock, err := initState(bstream.NewBlockRef("60a", 60), dirName, false, logger)
	require.NoError(t, err)

	blocks, reachedLib := forkDB.CompleteSegment(bstream.NewBlockRef("60a", 60))
	assert.True(t, reachedLib)
	require.Equal(t, 0, len(blocks))

	blocks, reachedLib = forkDB.CompleteSegment(bstream.NewBlockRef("105a", 105))
	assert.False(t, reachedLib)
	require.Equal(t, 0, len(blocks))

	assert.Equal(t, bstream.NewBlockRef("60a", 60), startBlock)
}

func assertForkableBlocks(t *testing.T, expected, actual []*forkable.Block) {
	t.Helper()

	require.Equal(t, len(expected), len(actual))
	for idx, expect := range expected {
		assertForkableBlock(t, expect, actual[idx])
	}
}

func assertForkableBlock(t *testing.T, expected, actual *forkable.Block) {
	t.Helper()
	assert.Equal(t, expected.BlockID, actual.BlockID)
	assert.Equal(t, expected.BlockNum, actual.BlockNum)
	assert.Equal(t, expected.PreviousBlockID, actual.PreviousBlockID)

	expectedBlock, ok := expected.Object.(*block)
	require.True(t, ok)
	actualBlock, ok := actual.Object.(*block)
	require.True(t, ok)

	assert.Equal(t, expectedBlock.fired, actualBlock.fired)
	assert.Equal(t, expectedBlock.Block.Id, actualBlock.Block.Id)
	assert.Equal(t, expectedBlock.Block.Number, actualBlock.Block.Number)
	assert.Equal(t, expectedBlock.Block.ParentId, actualBlock.Block.ParentId)
}
