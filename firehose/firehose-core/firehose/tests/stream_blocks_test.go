package firehose

//import (
//	"context"
//	"strings"
//	"testing"
//
//	pbfirehose "github.com/streamingfast/pbgo/sf/firehose/v1"
//
//	"github.com/streamingfast/bstream"
//	"github.com/streamingfast/dstore"
//	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
//	"github.com/stretchr/testify/assert"
//	"github.com/stretchr/testify/require"
//	"go.uber.org/zap"
//	"google.golang.org/protobuf/proto"
//)
//
//func TestLocalBlocks(t *testing.T) {
//
//	store := dstore.NewMockStore(nil)
//	idxStore := dstore.NewMockStore(nil)
//	blocksStores := []dstore.Store{store}
//	logger := zap.NewNop()
//
//	i := NewStreamFactory(
//		blocksStores,
//		idxStore,
//		[]uint64{10000, 1000, 100},
//		nil,
//		nil,
//		nil,
//	)
//
//	s := NewServer(
//		logger,
//		nil,
//		i,
//	)
//
//	// fake block decoder func to return bstream.Block
//	bstream.GetBlockDecoder = bstream.BlockDecoderFunc(func(blk *pbbstream.Block) (interface{}, error) {
//		block := new(pbbstream.Block)
//		block.Number = blk.Number
//		block.Id = blk.Id
//		block.PreviousId = blk.PreviousId
//		return block, nil
//	})
//
//	blocks := strings.Join([]string{
//		bstream.TestJSONBlockWithLIBNum("00000002a", "00000001a", 1),
//		bstream.TestJSONBlockWithLIBNum("00000003a", "00000002a", 2),
//		bstream.TestJSONBlockWithLIBNum("00000004a", "00000003a", 3), // last one closes on endblock
//	}, "\n")
//
//	store.SetFile("0000000000", []byte(blocks))
//
//	localClient := s.BlocksFromLocal(context.Background(), &pbfirehose.Request{
//		StartBlockNum: 2,
//		StopBlockNum:  4,
//	})
//
//	// ----
//	blk, err := localClient.Recv()
//	require.NoError(t, err)
//	b := &pbbstream.Block{}
//	err = proto.Unmarshal(blk.Block.Value, b)
//	require.NoError(t, err)
//	require.Equal(t, uint64(2), b.Number)
//	require.Equal(t, blk.Step, pbfirehose.ForkStep_STEP_NEW)
//
//	// ----
//	blk, err = localClient.Recv()
//	require.NoError(t, err)
//	b = &pbbstream.Block{}
//	err = proto.Unmarshal(blk.Block.Value, b)
//	require.NoError(t, err)
//	assert.Equal(t, uint64(3), b.Number)
//	assert.Equal(t, blk.Step, pbfirehose.ForkStep_STEP_NEW)
//
//	// ----
//	blk, err = localClient.Recv()
//	require.NoError(t, err)
//	b = &pbbstream.Block{}
//	err = proto.Unmarshal(blk.Block.Value, b)
//	require.NoError(t, err)
//	assert.Equal(t, uint64(2), b.Number)
//	assert.Equal(t, blk.Step, pbfirehose.ForkStep_STEP_IRREVERSIBLE)
//
//	// ----
//	blk, err = localClient.Recv()
//	require.NoError(t, err)
//	b = &pbbstream.Block{}
//	err = proto.Unmarshal(blk.Block.Value, b)
//	require.NoError(t, err)
//	assert.Equal(t, uint64(4), b.Number)
//	assert.Equal(t, blk.Step, pbfirehose.ForkStep_STEP_NEW)
//
//	// ----
//	blk, err = localClient.Recv()
//	require.NoError(t, err)
//	require.Nil(t, blk)
//}
