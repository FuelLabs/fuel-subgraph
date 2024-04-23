package firecore

import (
	"encoding/base64"
	"encoding/hex"
	"fmt"
	"testing"
	"time"

	"github.com/streamingfast/firehose-core/test"
	"github.com/streamingfast/logging"
	"github.com/stretchr/testify/require"
	"google.golang.org/protobuf/types/known/anypb"
)

var zlogTest, tracerTest = logging.PackageLogger("test", "github.com/streamingfast/firehose-core/firecore")

func Test_Ctx_readBlock(t *testing.T) {
	reader := &ConsoleReader{
		logger: zlogTest,
		tracer: tracerTest,

		readerProtocolVersion: "1.0",
		protoMessageType:      "type.googleapis.com/sf.ethereum.type.v2.Block",
	}

	blockHash := "d2836a703a02f3ca2a13f05efe26fc48c6fa0db0d754a49e56b066d3b7d54659"
	blockHashBytes, err := hex.DecodeString(blockHash)
	blockNumber := uint64(18571000)

	parentHash := "55de88c909fa368ae1e93b6b8ffb3fbb12e64aefec1d4a1fcc27ae7633de2f81"
	parentBlockNumber := 18570999

	libNumber := 18570800

	pbBlock := test.Block{
		Hash:   blockHashBytes,
		Number: blockNumber,
	}

	anypbBlock, err := anypb.New(&pbBlock)

	require.NoError(t, err)
	nowNano := time.Now().UnixNano()
	line := fmt.Sprintf(
		"%d %s %d %s %d %d %s",
		blockNumber,
		blockHash,
		parentBlockNumber,
		parentHash,
		libNumber,
		nowNano,
		base64.StdEncoding.EncodeToString(anypbBlock.Value),
	)

	block := reader.readBlock(line)
	require.NoError(t, err)

	require.Equal(t, blockNumber, block.Number)
	require.Equal(t, blockHash, block.Id)
	require.Equal(t, parentHash, block.ParentId)
	require.Equal(t, uint64(libNumber), block.LibNum)
	require.Equal(t, int32(time.Unix(0, nowNano).Nanosecond()), block.Timestamp.Nanos)

	require.NoError(t, err)
	require.Equal(t, anypbBlock.GetValue(), block.Payload.Value)

}

func Test_GetNext(t *testing.T) {
	lines := make(chan string, 2)
	reader := newConsoleReader(lines, zlogTest, tracerTest)

	initLine := "FIRE INIT 1.0 sf.ethereum.type.v2.Block"
	blockLine := "FIRE BLOCK 18571000 d2836a703a02f3ca2a13f05efe26fc48c6fa0db0d754a49e56b066d3b7d54659 18570999 55de88c909fa368ae1e93b6b8ffb3fbb12e64aefec1d4a1fcc27ae7633de2f81 18570800 1699992393935935000 Ci10eXBlLmdvb2dsZWFwaXMuY29tL3NmLmV0aGVyZXVtLnR5cGUudjIuQmxvY2sSJxIg0oNqcDoC88oqE/Be/ib8SMb6DbDXVKSeVrBm07fVRlkY+L3tCA=="

	lines <- initLine
	lines <- blockLine
	close(lines)

	block, err := reader.ReadBlock()
	require.NoError(t, err)

	require.Equal(t, uint64(18571000), block.Number)
	require.Equal(t, "d2836a703a02f3ca2a13f05efe26fc48c6fa0db0d754a49e56b066d3b7d54659", block.Id)
	require.Equal(t, "55de88c909fa368ae1e93b6b8ffb3fbb12e64aefec1d4a1fcc27ae7633de2f81", block.ParentId)
	require.Equal(t, uint64(18570800), block.LibNum)
	require.Equal(t, int32(time.Unix(0, 1699992393935935000).Nanosecond()), block.Timestamp.Nanos)
}
