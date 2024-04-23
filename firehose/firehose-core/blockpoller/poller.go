package blockpoller

import (
	"context"
	"fmt"
	"math"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/bstream/forkable"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/streamingfast/derr"
	"github.com/streamingfast/dhammer"
	"github.com/streamingfast/firehose-core/internal/utils"
	"github.com/streamingfast/shutter"
	"go.uber.org/zap"
)

type block struct {
	*pbbstream.Block
	fired bool
}

func newBlock(block2 *pbbstream.Block) *block {
	return &block{block2, false}
}

type BlockPoller struct {
	*shutter.Shutter
	startBlockNumGate        uint64
	fetchBlockRetryCount     uint64
	stateStorePath           string
	ignoreCursor             bool
	forceFinalityAfterBlocks *uint64

	blockFetcher BlockFetcher
	blockHandler BlockHandler
	forkDB       *forkable.ForkDB

	logger *zap.Logger

	optimisticallyPolledBlocks map[uint64]*BlockItem
}

func New(
	blockFetcher BlockFetcher,
	blockHandler BlockHandler,
	opts ...Option,
) *BlockPoller {

	b := &BlockPoller{
		Shutter:                  shutter.New(),
		blockFetcher:             blockFetcher,
		blockHandler:             blockHandler,
		fetchBlockRetryCount:     math.MaxUint64,
		logger:                   zap.NewNop(),
		forceFinalityAfterBlocks: utils.GetEnvForceFinalityAfterBlocks(),
	}

	for _, opt := range opts {
		opt(b)
	}

	return b
}

func (p *BlockPoller) Run(ctx context.Context, startBlockNum uint64, blockFetchBatchSize int) error {
	p.startBlockNumGate = startBlockNum
	p.logger.Info("starting poller",
		zap.Uint64("start_block_num", startBlockNum),
		zap.Uint64("block_fetch_batch_size", uint64(blockFetchBatchSize)),
	)
	p.blockHandler.Init()

	for {
		startBlock, skip, err := p.blockFetcher.Fetch(ctx, startBlockNum)
		if err != nil {
			return fmt.Errorf("unable to fetch start block %d: %w", startBlockNum, err)
		}
		if skip {
			startBlockNum++
			continue
		}
		return p.run(startBlock.AsRef(), blockFetchBatchSize)
	}
}

func (p *BlockPoller) run(resolvedStartBlock bstream.BlockRef, numberOfBlockToFetch int) (err error) {
	p.forkDB, resolvedStartBlock, err = initState(resolvedStartBlock, p.stateStorePath, p.ignoreCursor, p.logger)
	if err != nil {
		return fmt.Errorf("unable to initialize cursor: %w", err)
	}

	currentCursor := &cursor{state: ContinuousSegState, logger: p.logger}
	blockToFetch := resolvedStartBlock.Num()
	var hashToFetch *string
	for {
		if p.IsTerminating() {
			p.logger.Info("block poller is terminating")
		}

		p.logger.Info("about to fetch block", zap.Uint64("block_to_fetch", blockToFetch))
		var fetchedBlock *pbbstream.Block
		if hashToFetch != nil {
			fetchedBlock, err = p.fetchBlockWithHash(blockToFetch, *hashToFetch)
		} else {
			fetchedBlock, err = p.fetchBlock(blockToFetch, numberOfBlockToFetch)
		}

		if err != nil {
			return fmt.Errorf("unable to fetch  block %d: %w", blockToFetch, err)
		}

		blockToFetch, hashToFetch, err = p.processBlock(currentCursor, fetchedBlock)
		if err != nil {
			return fmt.Errorf("unable to fetch  block %d: %w", blockToFetch, err)
		}

		if p.IsTerminating() {
			p.logger.Info("block poller is terminating")
		}
	}
}

func (p *BlockPoller) processBlock(currentState *cursor, block *pbbstream.Block) (uint64, *string, error) {
	p.logger.Info("processing block", zap.Stringer("block", block.AsRef()), zap.Uint64("lib_num", block.LibNum))
	if block.Number < p.forkDB.LIBNum() {
		panic(fmt.Errorf("unexpected error block %d is below the current LIB num %d. There should be no re-org above the current LIB num", block.Number, p.forkDB.LIBNum()))
	}

	// On the first run, we will fetch the blk for the `startBlockRef`, since we have a `Ref` it stands
	// to reason that we may already have the block. We could potentially optimize this

	seenBlk, seenParent := p.forkDB.AddLink(block.AsRef(), block.ParentId, newBlock(block))

	currentState.addBlk(block, seenBlk, seenParent)

	blkCompleteSegNum := currentState.getBlkSegmentNum()
	completeSegment, reachLib := p.forkDB.CompleteSegment(blkCompleteSegNum)
	p.logger.Debug("checked if block is complete segment",
		zap.Uint64("blk_num", blkCompleteSegNum.Num()),
		zap.Int("segment_len", len(completeSegment)),
		zap.Bool("reached_lib", reachLib),
	)

	if reachLib {
		currentState.blkIsConnectedToLib()
		err := p.fireCompleteSegment(completeSegment)
		if err != nil {
			return 0, nil, fmt.Errorf("firing complete segment: %w", err)
		}

		// since the block is linkable to the current lib
		// we can safely set the new lib to the current block's Lib
		// the assumption here is that teh Lib the Block we received from the block fetcher ir ALWAYS CORRECT
		p.logger.Debug("setting lib", zap.Stringer("blk", block.AsRef()), zap.Uint64("lib_num", block.LibNum))
		p.forkDB.SetLIB(block.AsRef(), block.LibNum)
		p.forkDB.PurgeBeforeLIB(0)

		err = p.saveState(completeSegment)
		if err != nil {
			return 0, nil, fmt.Errorf("saving state: %w", err)
		}

		nextBlockNum := nextBlkInSeg(completeSegment)
		return nextBlockNum, nil, nil
	}

	currentState.blkIsNotConnectedToLib()

	prevBlockNum, prevBlockHash := prevBlockInSegment(completeSegment)
	return prevBlockNum, prevBlockHash, nil
}

type BlockItem struct {
	blockNumber uint64
	block       *pbbstream.Block
	skipped     bool
}

func (p *BlockPoller) loadNextBlocks(requestedBlock uint64, numberOfBlockToFetch int) error {
	p.optimisticallyPolledBlocks = map[uint64]*BlockItem{}

	nailer := dhammer.NewNailer(10, func(ctx context.Context, blockToFetch uint64) (*BlockItem, error) {
		var blockItem *BlockItem
		err := derr.Retry(p.fetchBlockRetryCount, func(ctx context.Context) error {
			b, skip, err := p.blockFetcher.Fetch(ctx, blockToFetch)
			if err != nil {
				return fmt.Errorf("unable to fetch  block %d: %w", blockToFetch, err)
			}
			if skip {
				blockItem = &BlockItem{
					blockNumber: blockToFetch,
					block:       nil,
					skipped:     true,
				}
				return nil
			}
			//todo: add block to cache
			blockItem = &BlockItem{
				blockNumber: blockToFetch,
				block:       b,
				skipped:     false,
			}
			return nil

		})

		if err != nil {
			return nil, fmt.Errorf("failed to fetch block with retries %d: %w", blockToFetch, err)
		}

		return blockItem, err
	})

	ctx := context.Background()
	nailer.Start(ctx)

	done := make(chan interface{}, 1)
	go func() {
		for blockItem := range nailer.Out {
			p.optimisticallyPolledBlocks[blockItem.blockNumber] = blockItem
		}

		close(done)
	}()

	didTriggerFetch := false
	for i := 0; i < numberOfBlockToFetch; i++ {
		b := requestedBlock + uint64(i)

		//only fetch block if it is available on chain
		if p.blockFetcher.IsBlockAvailable(b) {
			p.logger.Info("optimistically fetching block", zap.Uint64("block_num", b))
			didTriggerFetch = true
			nailer.Push(ctx, b)
		} else {
			//if this block is not available, we can assume that the next blocks are not available as well
			break
		}
	}

	if !didTriggerFetch {
		//if we did not trigger any fetch, we fetch the requested block
		// Fetcher should return the block when available (this will be a blocking call until the block is available)
		nailer.Push(ctx, requestedBlock)
	}

	nailer.Close()

	<-done

	if nailer.Err() != nil {
		return fmt.Errorf("failed optimistically fetch blocks starting at %d: %w", requestedBlock, nailer.Err())
	}

	return nil
}

func (p *BlockPoller) fetchBlock(blockNumber uint64, numberOfBlockToFetch int) (*pbbstream.Block, error) {
	for {
		blockItem, found := p.optimisticallyPolledBlocks[blockNumber]
		if !found {
			err := p.loadNextBlocks(blockNumber, numberOfBlockToFetch)
			if err != nil {
				return nil, fmt.Errorf("failed to load next blocks: %w", err)
			}
			continue //that will retry the current block after loading the more blocks
		}
		if blockItem.skipped {
			blockNumber++
			continue
		}

		p.logger.Info("block was optimistically polled", zap.Uint64("block_num", blockNumber))
		return blockItem.block, nil
	}
}

func (p *BlockPoller) fetchBlockWithHash(blkNum uint64, hash string) (*pbbstream.Block, error) {
	p.logger.Info("fetching block with hash", zap.Uint64("block_num", blkNum), zap.String("hash", hash))
	_ = hash //todo: hash will be used to fetch block from  cache

	p.optimisticallyPolledBlocks = map[uint64]*BlockItem{}

	var out *pbbstream.Block
	var skipped bool
	err := derr.Retry(p.fetchBlockRetryCount, func(ctx context.Context) error {
		//todo: get block from cache
		var fetchErr error
		out, skipped, fetchErr = p.blockFetcher.Fetch(ctx, blkNum)
		if fetchErr != nil {
			return fmt.Errorf("unable to fetch  block %d: %w", blkNum, fetchErr)
		}
		if skipped {
			return nil
		}
		return nil
	})

	if err != nil {
		return nil, fmt.Errorf("failed to fetch block with retries %d: %w", blkNum, err)
	}

	if skipped {
		return nil, fmt.Errorf("block %d was skipped and sould not have been requested", blkNum)
	}

	if p.forceFinalityAfterBlocks != nil {
		utils.TweakBlockFinality(out, *p.forceFinalityAfterBlocks)
	}

	return out, nil
}

func (p *BlockPoller) fireCompleteSegment(blocks []*forkable.Block) error {
	for _, blk := range blocks {
		b := blk.Object.(*block)
		if _, err := p.fire(b); err != nil {
			return fmt.Errorf("fireing block %d (%qs) %w", blk.BlockNum, blk.BlockID, err)
		}
	}
	return nil
}

func (p *BlockPoller) fire(blk *block) (bool, error) {
	if blk.fired {
		return false, nil
	}

	if blk.Number < p.startBlockNumGate {
		return false, nil
	}

	if err := p.blockHandler.Handle(blk.Block); err != nil {
		return false, err
	}

	blk.fired = true
	return true, nil
}

func nextBlkInSeg(blocks []*forkable.Block) uint64 {
	if len(blocks) == 0 {
		panic(fmt.Errorf("the blocks segments should never be empty"))
	}
	return blocks[len(blocks)-1].BlockNum + 1
}

func prevBlockInSegment(blocks []*forkable.Block) (uint64, *string) {
	if len(blocks) == 0 {
		panic(fmt.Errorf("the blocks segments should never be empty"))
	}
	blockObject := blocks[0].Object.(*block)
	return blockObject.ParentNum, &blockObject.ParentId
}
