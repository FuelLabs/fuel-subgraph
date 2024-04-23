package blockpoller

import (
	"errors"
	"fmt"
	"strconv"
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/bstream/forkable"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/stretchr/testify/assert"
)

func TestForkHandler_run(t *testing.T) {
	tests := []struct {
		name            string
		startBlock      bstream.BlockRef
		blocks          []*TestBlock
		expectFireBlock []*pbbstream.Block
	}{
		{
			name:       "start block 0",
			startBlock: blk("0a", "", 0).AsRef(),
			blocks: []*TestBlock{
				tb("0a", "", 0),
				tb("1a", "0a", 0),
				tb("2a", "1a", 0),
			},
			expectFireBlock: []*pbbstream.Block{
				blk("0a", "", 0),
				blk("1a", "0a", 0),
				blk("2a", "1a", 0),
			},
		},
		{
			name:       "Fork 1",
			startBlock: blk("100a", "99a", 100).AsRef(),
			blocks: []*TestBlock{
				tb("100a", "99a", 100),
				tb("101a", "100a", 100),
				tb("102a", "101a", 100),
				tb("103a", "102a", 100),
				tb("104b", "103b", 100),
				tb("103a", "102a", 100),
				tb("104a", "103a", 100),
				tb("105b", "104b", 100),
				tb("103b", "102b", 100),
				tb("102b", "101a", 100),
				tb("106a", "105a", 100),
				tb("105a", "104a", 100),
			},
			expectFireBlock: []*pbbstream.Block{
				blk("100a", "99a", 100),
				blk("101a", "100a", 100),
				blk("102a", "101a", 100),
				blk("103a", "102a", 100),
				blk("104a", "103a", 100),
				blk("102b", "101a", 100),
				blk("103b", "102b", 100),
				blk("104b", "103b", 100),
				blk("105b", "104b", 100),
				blk("105a", "104a", 100),
				blk("106a", "105a", 100),
			},
		},
		{
			name:       "Fork 2",
			startBlock: blk("100a", "99a", 100).AsRef(),
			blocks: []*TestBlock{
				tb("100a", "99a", 100),
				tb("101a", "100a", 100),
				tb("102a", "101a", 100),
				tb("103a", "102a", 100),
				tb("104b", "103b", 100),
				tb("103a", "102a", 100),
				tb("104a", "103a", 100),
				tb("105b", "104b", 100),
				tb("103b", "102b", 100),
				tb("102a", "101a", 100),
				tb("103a", "104a", 100),
				tb("104a", "105a", 100),
				tb("105a", "104a", 100),
			},
			expectFireBlock: []*pbbstream.Block{
				blk("100a", "99a", 100),
				blk("101a", "100a", 100),
				blk("102a", "101a", 100),
				blk("103a", "102a", 100),
				blk("104a", "103a", 100),
				blk("105a", "104a", 100),
			},
		},
		{
			name:       "with lib advancing",
			startBlock: blk("100a", "99a", 100).AsRef(),
			blocks: []*TestBlock{
				tb("100a", "99a", 100),
				tb("101a", "100a", 100),
				tb("102a", "101a", 100),
				tb("103a", "102a", 101),
				tb("104b", "103b", 101),
				tb("103a", "102a", 101),
				tb("104a", "103a", 101),
				tb("105b", "104b", 101),
				tb("103b", "102b", 101),
				tb("102a", "101a", 101),
				tb("103a", "104a", 101),
				tb("104a", "105a", 101),
				tb("105a", "104a", 101),
			},
			expectFireBlock: []*pbbstream.Block{
				blk("100a", "99a", 100),
				blk("101a", "100a", 100),
				blk("102a", "101a", 100),
				blk("103a", "102a", 100),
				blk("104a", "103a", 100),
				blk("105a", "104a", 100),
			},
		},
		{
			name:       "with skipping blocks",
			startBlock: blk("100a", "99a", 100).AsRef(),
			blocks: []*TestBlock{
				tb("100a", "99a", 100),
				tb("101a", "100a", 100),
				tb("102a", "101a", 100),
				tb("103a", "102a", 101),
				tb("104b", "103b", 101),
				tb("103a", "102a", 101),
				tb("104a", "103a", 101),
				tb("105b", "104b", 101),
				tb("103b", "102b", 101),
				tb("102a", "101a", 101),
				tb("103a", "104a", 101),
				tb("104a", "105a", 101),
				tb("105a", "104a", 101),
			},
			expectFireBlock: []*pbbstream.Block{
				blk("100a", "99a", 100),
				blk("101a", "100a", 100),
				blk("102a", "101a", 100),
				blk("103a", "102a", 100),
				blk("104a", "103a", 100),
				blk("105a", "104a", 100),
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {

			blockFetcher := newTestBlockFetcher(t, tt.blocks)
			blockFinalizer := newTestBlockFinalizer(t, tt.expectFireBlock)

			f := New(blockFetcher, blockFinalizer)
			f.forkDB = forkable.NewForkDB()

			err := f.run(tt.startBlock, 1)
			if !errors.Is(err, TestErrCompleteDone) {
				require.NoError(t, err)
			}

			blockFetcher.check(t)
			blockFinalizer.check(t)

		})
	}
}

func TestForkHandler_fire(t *testing.T) {
	tests := []struct {
		name          string
		block         *block
		startBlockNum uint64
		expect        bool
	}{
		{
			name:          "greater then start block",
			block:         &block{blk("100a", "99a", 98), false},
			startBlockNum: 98,
			expect:        true,
		},
		{
			name:          "on then start block",
			block:         &block{blk("100a", "99a", 98), false},
			startBlockNum: 100,
			expect:        true,
		},
		{
			name:          "less then start block",
			block:         &block{blk("100a", "99a", 98), false},
			startBlockNum: 101,
			expect:        false,
		},
		{
			name:          "already fired",
			block:         &block{blk("100a", "99a", 98), true},
			startBlockNum: 98,
			expect:        false,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			poller := &BlockPoller{startBlockNumGate: test.startBlockNum, blockHandler: &TestNoopBlockFinalizer{}}
			ok, err := poller.fire(test.block)
			require.NoError(t, err)
			assert.Equal(t, test.expect, ok)
		})
	}

}

func tb(id, prev string, libNum uint64) *TestBlock {
	return &TestBlock{
		expect: blk(id, prev, libNum),
		send:   blk(id, prev, libNum),
	}
}

func blk(id, prev string, libNum uint64) *pbbstream.Block {
	return &pbbstream.Block{
		Number:    blockNum(id),
		Id:        id,
		ParentId:  prev,
		LibNum:    libNum,
		ParentNum: blockNum(prev),
	}
}

func forkBlk(id string) *forkable.Block {
	return &forkable.Block{
		BlockID:  id,
		BlockNum: blockNum(id),
		Object: &block{
			Block: &pbbstream.Block{
				Number: blockNum(id),
				Id:     id,
			},
		},
	}
}

func blockNum(blockID string) uint64 {
	b := blockID
	if len(blockID) < 8 { // shorter version, like 8a for 00000008a
		b = fmt.Sprintf("%09s", blockID)
	}
	bin, err := strconv.ParseUint(b[:8], 10, 64)
	if err != nil {
		panic(err)
	}
	return bin
}
