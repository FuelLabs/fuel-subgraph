package types

import (
	"fmt"
	"math"

	"github.com/streamingfast/bstream"
)

//go:generate go-enum -f=$GOFILE --marshal --names --nocase

// ENUM(
//
//	Inclusive
//	Exclusive
//
// )
type RangeBoundary int

const (
	EndBoundaryInclusive RangeBoundary = RangeBoundaryInclusive
	EndBoundaryExclusive               = RangeBoundaryExclusive
)

// BlockRange is actually an UnresolvedBlockRange so both the start and end could be
// negative values.
//
// This is in opposition to `bstream.Range` which is a resolved range meaning that start/stop
// values will never be negative.
type BlockRange struct {
	Start int64
	Stop  *uint64
}

func NewOpenRange(start int64) BlockRange {
	return BlockRange{Start: int64(start), Stop: nil}
}

func NewClosedRange(start int64, stop uint64) BlockRange {
	return BlockRange{Start: start, Stop: &stop}
}

// IsResolved returns true if the range is both closed and fully
// resolved (e.g. both start and stop are positive values). Returns
// false otherwise.
func (b BlockRange) IsResolved() bool {
	return b.Start >= 0 && b.IsClosed()
}

func (b BlockRange) IsOpen() bool {
	return b.Stop == nil
}

func (b BlockRange) IsClosed() bool {
	return b.Stop != nil
}

func (b BlockRange) GetStartBlock() int64 {
	return b.Start
}

func (b BlockRange) BlockCount() int64 {
	if !b.IsResolved() {
		return math.MaxInt64
	}

	return int64(*b.Stop) - b.Start + 1
}

func (b BlockRange) GetStopBlockOr(defaultIfOpenRange uint64) uint64 {
	if b.IsOpen() {
		return defaultIfOpenRange
	}

	return *b.Stop
}

func (b BlockRange) ReprocRange() string {
	if !b.IsClosed() {
		return "<Invalid Unbounded Range>"
	}

	if !b.IsResolved() {
		return "<Invalid Unresolved Range>"
	}

	return fmt.Sprintf("%d:%d", b.Start, *b.Stop+1)
}

func (b BlockRange) Contains(blockNum uint64, endBoundary RangeBoundary) bool {
	if blockNum < uint64(b.Start) {
		return false
	}

	if b.Stop == nil {
		return true
	}

	endBlock := *b.Stop
	if blockNum > endBlock {
		return false
	}
	if endBoundary == RangeBoundaryExclusive && blockNum == endBlock {
		return false
	}

	return true
}

func (b BlockRange) Split(chunkSize uint64, endBoundary RangeBoundary) ([]BlockRange, error) {
	segments, err := b.ToBstreamRange(endBoundary).Split(chunkSize)
	if err != nil {
		return nil, fmt.Errorf("splitting ranges: %w", err)
	}

	out := make([]BlockRange, len(segments))
	for i, segment := range segments {
		out[i] = NewClosedRange(int64(segment.StartBlock()), *segment.EndBlock())
	}

	return out, nil
}

func (b BlockRange) ToBstreamRange(endBoundary RangeBoundary) *bstream.Range {
	if b.Start < 0 {
		panic(fmt.Errorf("cannot convert unresolved block range to bstream.Range: %s", b))
	}

	if b.IsOpen() {
		return bstream.NewOpenRange(uint64(b.Start))
	}

	if endBoundary == RangeBoundaryExclusive {
		return bstream.NewRangeExcludingEnd(uint64(b.Start), *b.Stop)
	}

	return bstream.NewInclusiveRange(uint64(b.Start), *b.Stop)
}

func (b BlockRange) String() string {
	if b.IsOpen() {
		return fmt.Sprintf("[%s, +∞]", BlockNum(b.Start))
	}

	return fmt.Sprintf("[%s, %s]", BlockNum(b.Start), BlockNum(*b.Stop))
}
