package blockpoller

import (
	"github.com/streamingfast/bstream"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"go.uber.org/zap"
)

type State string

const (
	ContinuousSegState State = "CONTINUOUS"
	IncompleteSegState State = "INCOMPLETE"
)

func (s State) String() string {
	return string(s)
}

type cursor struct {
	currentBlk           bstream.BlockRef
	currentIncompleteSeg *bstream.BasicBlockRef
	state                State
	logger               *zap.Logger
}

func (s *cursor) addBlk(blk *pbbstream.Block, blockSeen bool, parentSeen bool) {
	blkRef := blk.AsRef()
	logger := s.logger.With(
		zap.Stringer("blk", blkRef),
		zap.Stringer("parent_blk", blk.PreviousRef()),
		zap.Bool("seen_blk", blockSeen),
		zap.Bool("seen_parent", parentSeen),
		zap.Stringer("previous_state", s.state),
	)
	if s.currentIncompleteSeg != nil {
		logger = logger.With(zap.Stringer("current_incomplete_seg", *s.currentIncompleteSeg))
	} else {
		logger = logger.With(zap.String("current_incomplete_seg", "none"))

	}

	if s.state == IncompleteSegState && blockSeen && parentSeen {
		// if we are checking an incomplete segement, and we get a block that is already in the forkdb
		// and whose parent is also in the forkdb, then we are back on a continuous segment
		s.state = ContinuousSegState
	}
	s.currentBlk = blkRef
	logger.Debug("received block", zap.Stringer("current_state", s.state))
}

func (s *cursor) getBlkSegmentNum() bstream.BlockRef {
	if s.state == IncompleteSegState {
		if s.currentIncompleteSeg == nil {
			panic("current incomplete segment is nil, when cursor is incomplete segment, this should never happen")
		}
		return *s.currentIncompleteSeg
	}
	return s.currentBlk
}

func (s *cursor) blkIsConnectedToLib() {
	s.state = ContinuousSegState
	s.currentIncompleteSeg = nil
}

func (s *cursor) blkIsNotConnectedToLib() {
	if s.state != IncompleteSegState {
		s.state = IncompleteSegState
		// we don't want to point the current blk since that will change
		v := bstream.NewBlockRef(s.currentBlk.ID(), s.currentBlk.Num())
		s.currentIncompleteSeg = &v
	}
}
