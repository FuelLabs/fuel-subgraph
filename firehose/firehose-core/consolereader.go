package firecore

import (
	"encoding/hex"
	"fmt"
	"github.com/streamingfast/bstream"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/streamingfast/dmetrics"
	"github.com/streamingfast/firehose-core/node-manager/mindreader"
	"github.com/streamingfast/firehose-core/pb/sf/fuel/type/v1"
	"github.com/streamingfast/logging"
	"go.uber.org/zap"
	"google.golang.org/protobuf/proto"
	"google.golang.org/protobuf/types/known/anypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"io"
	"strings"
	"sync"
	"time"
)

const FirePrefix = "FIRE "
const FirePrefixLen = len(FirePrefix)
const FuelProtoPrefix = "PROTO "
const FuelProtoPrefixLen = len(FuelProtoPrefix)

type ParsingStats struct {
}

type ConsoleReader struct {
	lines     chan string
	done      chan interface{}
	closeOnce sync.Once
	logger    *zap.Logger
	tracer    logging.Tracer

	// Parsing context
	readerProtocolVersion string
	protoMessageType      string
	lastBlock             bstream.BlockRef
	lastParentBlock       bstream.BlockRef
	lastBlockTimestamp    time.Time

	lib uint64

	blockRate *dmetrics.AvgRatePromCounter
}

func NewConsoleReader(lines chan string, blockEncoder BlockEncoder, logger *zap.Logger, tracer logging.Tracer) (mindreader.ConsolerReader, error) {
	reader := newConsoleReader(lines, logger, tracer)

	delayBetweenStats := 30 * time.Second
	if tracer.Enabled() {
		delayBetweenStats = 5 * time.Second
	}

	go func() {
		defer reader.blockRate.Stop()

		for {
			select {
			case <-reader.done:
				return
			case <-time.After(delayBetweenStats):
				reader.printStats()
			}
		}
	}()

	return reader, nil
}

func newConsoleReader(lines chan string, logger *zap.Logger, tracer logging.Tracer) *ConsoleReader {
	return &ConsoleReader{
		lines:            lines,
		done:             make(chan interface{}),
		logger:           logger,
		tracer:           tracer,
		protoMessageType: "sf.fuel.type.v1.Block",
		blockRate:        dmetrics.MustNewAvgRateFromPromCounter(ConsoleReaderBlockReadCount, 1*time.Second, 30*time.Second, "blocks"),
	}
}

func (r *ConsoleReader) Done() <-chan interface{} {
	return r.done
}

func (r *ConsoleReader) Close() error {
	r.closeOnce.Do(func() {
		r.blockRate.SyncNow()
		r.printStats()

		r.logger.Info("console reader done")
		close(r.done)
	})

	return nil
}

type blockRefView struct {
	ref bstream.BlockRef
}

func (v blockRefView) String() string {
	if v.ref == nil {
		return "<unset>"
	}

	return v.ref.String()
}

type blockRefViewTimestamp struct {
	ref       bstream.BlockRef
	timestamp time.Time
}

func (v blockRefViewTimestamp) String() string {
	return fmt.Sprintf("%s @ %s", blockRefView{v.ref}, v.timestamp.Local().Format(time.RFC822Z))
}

func (r *ConsoleReader) printStats() {
	r.logger.Info("console reader stats",
		zap.Stringer("block_rate", r.blockRate),
		zap.Stringer("last_block", blockRefViewTimestamp{r.lastBlock, r.lastBlockTimestamp}),
		zap.Stringer("last_parent_block", blockRefView{r.lastParentBlock}),
		zap.Uint64("lib", r.lib),
	)
}

func (r *ConsoleReader) ReadBlock() (out *pbbstream.Block, err error) {
	out, err = r.next()
	if err != nil {
		return nil, err
	}

	return out, nil
}

func (r *ConsoleReader) next() (out *pbbstream.Block, err error) {
	for line := range r.lines {
		if !strings.HasPrefix(line, FirePrefix) {
			continue
		}

		line = line[FirePrefixLen:]

		switch {
		case strings.HasPrefix(line, FuelProtoPrefix):
			out, err = r.readFuelProto(line[FuelProtoPrefixLen:])

		default:
			if r.tracer.Enabled() {
				r.logger.Debug("skipping unknown Firehose log line", zap.String("line", line))
			}
			continue
		}

		if err != nil {
			chunks := strings.SplitN(line, " ", 2)
			return nil, fmt.Errorf("%s: %s (line %q)", chunks[0], err, line)
		}

		if out != nil {
			return out, nil
		}
	}

	r.Close()

	return nil, io.EOF
}

func (r *ConsoleReader) readFuelProto(params string) (*pbbstream.Block, error) {

	fuelBlock := &pbfuel.Block{}

	out, err := hex.DecodeString(params)
	if err != nil {
		return nil, fmt.Errorf("read block %d: invalid string value: %w. Unable to decode the string", err)
	}

	if err := proto.Unmarshal(out, fuelBlock); err != nil {
		return nil, fmt.Errorf("read block %d: invalid proto: %w")
	}
	if err != nil {
		return nil, fmt.Errorf("unable to unmarshal decoded string: %w", err)
	}

	blockPayload := &anypb.Any{
		TypeUrl: r.protoMessageType,
		Value:   out,
	}

	block := &pbbstream.Block{
		Id:        fuelBlock.GetFirehoseBlockID(),
		Number:    fuelBlock.GetFirehoseBlockNumber(),
		ParentId:  fuelBlock.GetFirehoseBlockParentID(),
		ParentNum: fuelBlock.GetFirehoseBlockParentNumber(),
		Timestamp: timestamppb.New(fuelBlock.GetFirehoseBlockTime()),
		LibNum:    fuelBlock.GetFirehoseBlockLIBNum(),
		Payload:   blockPayload,
	}

	ConsoleReaderBlockReadCount.Inc()
	r.lastBlock = bstream.NewBlockRef(fuelBlock.GetFirehoseBlockID(), fuelBlock.GetFirehoseBlockNumber())
	r.lastParentBlock = bstream.NewBlockRef(fuelBlock.GetFirehoseBlockParentID(), fuelBlock.GetFirehoseBlockParentNumber())
	r.lastBlockTimestamp = fuelBlock.GetFirehoseBlockTime()
	r.lib = fuelBlock.GetFirehoseBlockLIBNum()

	return block, nil
}

func (r *ConsoleReader) setProtoMessageType(typeURL string) {
	if strings.HasPrefix(typeURL, "type.googleapis.com/") {
		r.protoMessageType = typeURL
		return
	}

	if strings.Contains(typeURL, "/") {
		panic(fmt.Sprintf("invalid type url %q, expecting type.googleapis.com/", typeURL))
	}

	r.protoMessageType = "type.googleapis.com/" + typeURL
}

// splitInBoundedChunks splits the line in `count` chunks and returns the slice `chunks[1:count]` (so exclusive end),
// but will accumulate all trailing chunks within the last (for free-form strings, or JSON objects)
func splitInBoundedChunks(line string, count int) ([]string, error) {
	chunks := strings.SplitN(line, " ", count)
	if len(chunks) != count {
		return nil, fmt.Errorf("%d fields required but found %d fields for line %q", count, len(chunks), line)
	}

	return chunks, nil
}
