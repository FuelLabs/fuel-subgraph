package merger

import (
	//	"context"
	//"fmt"

	"context"
	"testing"

	//	"time"

	//	"github.com/streamingfast/bstream"
	//"github.com/streamingfast/firehose-core/merger/bundle"
	"github.com/streamingfast/bstream"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setPbBlock(obf *bstream.OneBlockFile) {
	//pbb := &pbbstream.Block{
	//	Number: obf.Num,
	//}
	//out, err := proto.Marshal(pbb)
	//if err != nil {
	//	panic(err)
	//}
	//obf.MemoizeData = out
}

var block98 = func() *bstream.OneBlockFile {
	obf := bstream.MustNewOneBlockFile("0000000098-0000000000000098a-0000000000000097a-96-suffix")
	setPbBlock(obf)
	return obf
}
var block99 = func() *bstream.OneBlockFile {
	obf := bstream.MustNewOneBlockFile("0000000099-0000000000000099a-0000000000000098a-97-suffix")
	setPbBlock(obf)
	return obf

}
var block100 = func() *bstream.OneBlockFile {
	obf := bstream.MustNewOneBlockFile("0000000100-0000000000000100a-0000000000000099a-98-suffix")
	setPbBlock(obf)
	return obf
}
var block101 = func() *bstream.OneBlockFile {
	obf := bstream.MustNewOneBlockFile("0000000101-0000000000000101a-0000000000000100a-99-suffix")
	setPbBlock(obf)
	return obf
}
var block102Final100 = func() *bstream.OneBlockFile {
	obf := bstream.MustNewOneBlockFile("0000000102-0000000000000102a-0000000000000101a-100-suffix")
	setPbBlock(obf)
	return obf
}
var block103Final101 = func() *bstream.OneBlockFile {
	obf := bstream.MustNewOneBlockFile("0000000103-0000000000000103a-0000000000000102a-101-suffix")
	setPbBlock(obf)
	return obf
}
var block104Final102 = func() *bstream.OneBlockFile {
	obf := bstream.MustNewOneBlockFile("0000000104-0000000000000104a-0000000000000103a-102-suffix")
	setPbBlock(obf)
	return obf
}
var block105Final103 = func() *bstream.OneBlockFile {
	obf := bstream.MustNewOneBlockFile("0000000105-0000000000000105a-0000000000000104a-103-suffix")
	setPbBlock(obf)
	return obf
}
var block106Final104 = func() *bstream.OneBlockFile {
	obf := bstream.MustNewOneBlockFile("0000000106-0000000000000106a-0000000000000105a-104-suffix")
	setPbBlock(obf)
	return obf
}

var block507Final106 = func() *bstream.OneBlockFile {
	obf := bstream.MustNewOneBlockFile("0000000507-0000000000000507a-0000000000000106a-106-suffix")
	setPbBlock(obf)
	return obf
}
var block608Final507 = func() *bstream.OneBlockFile {
	obf := bstream.MustNewOneBlockFile("0000000608-0000000000000608a-0000000000000507a-507-suffix")
	setPbBlock(obf)
	return obf
}
var block609Final608 = func() *bstream.OneBlockFile {
	obf := bstream.MustNewOneBlockFile("0000000609-0000000000000609a-0000000000000608a-608-suffix")
	setPbBlock(obf)
	return obf
}

func TestNewBundler(t *testing.T) {
	b := NewBundler(100, 200, 2, 100, nil)
	require.NotNil(t, b)
	assert.EqualValues(t, 100, b.bundleSize)
	assert.EqualValues(t, 200, b.stopBlock)
	assert.NotNil(t, b.bundleError)
	assert.NotNil(t, b.seenBlockFiles)
}

func TestBundlerReset(t *testing.T) {
	b := NewBundler(100, 200, 2, 2, nil) // merge every 2 blocks

	b.irreversibleBlocks = []*bstream.OneBlockFile{block100(), block101()}
	b.Reset(102, block100().ToBstreamBlock().AsRef())
	assert.Nil(t, b.irreversibleBlocks)
	assert.EqualValues(t, 102, b.baseBlockNum)

}

func TestBundlerMergeKeepOne(t *testing.T) {

	tests := []struct {
		name            string
		inBlocks        []*bstream.OneBlockFile
		mergeSize       uint64
		expectRemaining []*bstream.OneBlockFile
		expectBase      uint64
		expectMerged    []uint64
	}{
		{
			name: "vanilla",
			inBlocks: []*bstream.OneBlockFile{
				block100(),
				block101(),
				block102Final100(),
				block103Final101(),
				block104Final102(),
			},
			mergeSize: 2,
			expectRemaining: []*bstream.OneBlockFile{
				block101(),
				block102Final100(),
			},
			expectBase:   102,
			expectMerged: []uint64{100},
		},
		{
			name: "vanilla_plus_one",
			inBlocks: []*bstream.OneBlockFile{
				block100(),
				block101(),
				block102Final100(),
				block103Final101(),
				block104Final102(),
				block105Final103(),
			},
			mergeSize: 2,
			expectRemaining: []*bstream.OneBlockFile{
				block101(),
				block102Final100(),
				block103Final101(),
			},
			expectBase:   102,
			expectMerged: []uint64{100},
		},
		{
			name: "twoMerges",
			inBlocks: []*bstream.OneBlockFile{
				block100(),
				block101(),
				block102Final100(),
				block103Final101(),
				block104Final102(),
				block105Final103(),
				block106Final104(),
			},
			mergeSize: 2,
			expectRemaining: []*bstream.OneBlockFile{
				block103Final101(),
				block104Final102(),
			},
			expectBase:   104,
			expectMerged: []uint64{100, 102},
		},
		{
			name: "big_hole",
			inBlocks: []*bstream.OneBlockFile{
				block100(),
				block101(),
				block102Final100(),
				block103Final101(),
				block104Final102(),
				block105Final103(),
				block106Final104(),
				block507Final106(),
				block608Final507(),
				block609Final608(),
			},
			mergeSize: 100,
			expectRemaining: []*bstream.OneBlockFile{
				block507Final106(), // last from bundle 500
				block608Final507(), // the only irreversible block from current bundle
			},
			expectBase:   600,
			expectMerged: []uint64{100, 200, 300, 400, 500},
		},
	}

	for _, c := range tests {

		t.Run(c.name, func(t *testing.T) {
			var merged []uint64
			b := NewBundler(100, 700, 2, c.mergeSize, &TestMergerIO{
				MergeAndStoreFunc: func(_ context.Context, inclusiveLowerBlock uint64, _ []*bstream.OneBlockFile) (err error) {
					merged = append(merged, inclusiveLowerBlock)
					return nil
				},
			}) // merge every 2 blocks
			b.irreversibleBlocks = []*bstream.OneBlockFile{block100(), block101()}

			for _, blk := range c.inBlocks {
				require.NoError(t, b.HandleBlockFile(blk))
			}

			// wait for MergeAndStore
			b.inProcess.Lock()
			b.inProcess.Unlock()

			assert.Equal(t, c.expectMerged, merged)
			assert.Equal(t, c.expectRemaining, b.irreversibleBlocks)
			assert.Equal(t, int(c.expectBase), int(b.baseBlockNum))
		})
	}
}
