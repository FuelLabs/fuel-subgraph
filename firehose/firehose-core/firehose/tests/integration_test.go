package firehose

//import (
//	"context"
//	"encoding/json"
//	"fmt"
//	"testing"
//	"time"
//
//	"github.com/alicebob/miniredis/v2/server"
//	"github.com/streamingfast/bstream"
//	"github.com/streamingfast/dstore"
//	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
//	pbfirehose "github.com/streamingfast/pbgo/sf/firehose/v1"
//	"github.com/stretchr/testify/assert"
//	"github.com/stretchr/testify/require"
//	"go.uber.org/zap"
//	"google.golang.org/protobuf/proto"
//)
//
//func TestFullFlow(t *testing.T) {
//
//	stepNew := pbfirehose.ForkStep_STEP_NEW
//	stepIrr := pbfirehose.ForkStep_STEP_IRREVERSIBLE
//	stepUndo := pbfirehose.ForkStep_STEP_UNDO
//	_ = stepUndo
//
//	type expectedResp struct {
//		num  uint64
//		id   string
//		step pbfirehose.ForkStep
//	}
//
//	tests := []struct {
//		name                      string
//		files                     map[int][]byte
//		irreversibleBlocksIndexes map[int]map[int]string
//		startBlockNum             uint64
//		stopBlockNum              uint64
//		cursor                    *bstream.LastFiredBlock
//		expectedResponses         []expectedResp
//	}{
//		{
//			"scenario 1 -- irreversible index, no cursor",
//			map[int][]byte{
//				0: testBlocks(
//					4, "4a", "3a", 0,
//					6, "6a", "4a", 0,
//				),
//				100: testBlocks(
//					100, "100a", "6a", 6,
//					102, "102a", "100a", 6,
//					103, "103a", "102a", 100, // moves LIB from 6 to 100
//				),
//				200: testBlocks(
//					204, "204b", "103a", 102, // moves LIB from 100 to 102
//					205, "205b", "103b", 100, //unlinkable
//				),
//			},
//			map[int]map[int]string{
//				0: {
//					4: "4a",
//					6: "6a",
//				},
//				200: { // this hould not be used
//					204: "204a",
//					206: "206a",
//				},
//			},
//			5,
//			0,
//			nil,
//			[]expectedResp{
//				{6, "6a", stepNew},
//				{6, "6a", stepIrr},
//				{100, "100a", stepNew},
//				{102, "102a", stepNew},
//				{103, "103a", stepNew},
//				{100, "100a", stepIrr},
//				{204, "204b", stepNew},
//				{102, "102a", stepIrr},
//			},
//		},
//		{
//			"scenario 2 -- no irreversible index, start->stop with some libs",
//			map[int][]byte{
//				0: testBlocks(
//					4, "4a", "3a", 0,
//					6, "6a", "4a", 4,
//				),
//				100: testBlocks(
//					100, "100a", "6a", 6,
//					102, "102a", "100a", 6,
//					103, "103a", "102a", 100, // triggers StepIrr
//					104, "104a", "103a", 100, // after stop block
//				),
//			},
//			nil,
//			6,
//			103,
//			nil,
//			[]expectedResp{
//				{6, "6a", stepNew},
//				{100, "100a", stepNew},
//				{6, "6a", stepIrr},
//				{102, "102a", stepNew},
//				{103, "103a", stepNew},
//			},
//		},
//	}
//
//	for _, c := range tests {
//		t.Run(c.name, func(t *testing.T) {
//
//			logger := zap.NewNop()
//			bs := dstore.NewMockStore(nil)
//			for i, data := range c.files {
//				bs.SetFile(base(i), data)
//			}
//
//			irrStore := getIrrStore(c.irreversibleBlocksIndexes)
//
//			// fake block decoder func to return pbbstream.Block
//			bstream.GetBlockDecoder = bstream.BlockDecoderFunc(func(blk *pbbstream.Block) (interface{}, error) {
//				block := new(pbbstream.Block)
//				block.Number = blk.Number
//				block.Id = blk.Id
//				block.PreviousId = blk.PreviousId
//				return block, nil
//			})
//
//			tracker := bstream.NewTracker(0) // 0 value not used
//			fmt.Println(bstream.GetProtocolFirstStreamableBlock)
//			tracker.AddResolver(bstream.OffsetStartBlockResolver(200))
//
//			i := NewStreamFactory(
//				[]dstore.Store{bs},
//				irrStore,
//				[]uint64{10000, 1000, 100},
//				nil,
//				nil,
//				tracker,
//			)
//
//			s := server.NewServer(
//				logger,
//				nil,
//				i,
//			)
//
//			ctx, cancelCtx := context.WithCancel(context.Background())
//			defer cancelCtx()
//			localClient := s.BlocksFromLocal(ctx, &pbfirehose.Request{
//				StartBlockNum: int64(c.startBlockNum),
//				StopBlockNum:  c.stopBlockNum,
//			})
//
//			for _, r := range c.expectedResponses {
//				resp, err := localClient.Recv()
//				require.NotNil(t, resp)
//				require.NoError(t, err)
//
//				fmt.Println(resp.LastFiredBlock)
//				cursor, err := bstream.CursorFromOpaque(resp.LastFiredBlock)
//				require.NoError(t, err, "cursor sent from firehose should always be valid")
//				require.False(t, cursor.IsEmpty())
//
//				b := &pbbstream.Block{}
//				err = proto.Unmarshal(resp.Block.Value, b)
//				require.NoError(t, err)
//
//				require.Equal(t, r.num, b.Number)
//				require.Equal(t, r.id, b.Id)
//				require.Equal(t, r.step, resp.Step)
//			}
//
//			// catchExtraBlock
//			moreChan := make(chan *pbbstream.Block)
//			go func() {
//				resp, err := localClient.Recv()
//				require.NoError(t, err)
//				if resp == nil {
//					return
//				}
//
//				b := &pbbstream.Block{}
//				err = proto.Unmarshal(resp.Block.Value, b)
//				require.NoError(t, err)
//				moreChan <- b
//			}()
//
//			select {
//			case resp := <-moreChan:
//				assert.Falsef(t, true, "an extra block was seen: %s", resp.String())
//			case <-time.After(time.Millisecond * 50):
//			}
//
//		})
//	}
//
//}
//
//func base(in int) string {
//	return fmt.Sprintf("%010d", in)
//}
//
//func testBlocks(in ...interface{}) (out []byte) {
//	var blks []bstream.ParsableTestBlock
//	for i := 0; i < len(in); i += 4 {
//		blks = append(blks, bstream.ParsableTestBlock{
//			Number:     uint64(in[i].(int)),
//			ID:         in[i+1].(string),
//			PreviousID: in[i+2].(string),
//			LIBNum:     uint64(in[i+3].(int)),
//		})
//	}
//
//	for _, blk := range blks {
//		b, err := json.Marshal(blk)
//		if err != nil {
//			panic(err)
//		}
//		out = append(out, b...)
//		out = append(out, '\n')
//	}
//	return
//}
//
//func getIrrStore(irrBlkIdxs map[int]map[int]string) (irrStore *dstore.MockStore) {
//	irrStore = dstore.NewMockStore(nil)
//	for j, n := range irrBlkIdxs {
//		filename, cnt := bstream.TestIrrBlocksIdx(j, 100, n)
//		irrStore.SetFile(filename, cnt)
//	}
//	return
//}
