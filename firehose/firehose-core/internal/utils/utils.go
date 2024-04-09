package utils

import (
	"os"
	"strconv"

	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
)

func GetEnvForceFinalityAfterBlocks() *uint64 {
	if fin := os.Getenv("FORCE_FINALITY_AFTER_BLOCKS"); fin != "" {
		if fin64, err := strconv.ParseInt(fin, 10, 64); err == nil {
			finu64 := uint64(fin64)
			return &finu64
		}
	}
	return nil
}

func TweakBlockFinality(blk *pbbstream.Block, maxDistanceToBlock uint64) {
	if blk.LibNum > blk.Number {
		panic("libnum cannot be greater than block number")
	}
	if blk.Number < maxDistanceToBlock {
		return // prevent uin64 underflow at the beginning of the chain
	}
	if (blk.Number - blk.LibNum) >= maxDistanceToBlock {
		blk.LibNum = blk.Number - maxDistanceToBlock // force finality
	}
}
