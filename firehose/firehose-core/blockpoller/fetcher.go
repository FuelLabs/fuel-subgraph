package blockpoller

import (
	"context"

	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
)

type BlockFetcher interface {
	IsBlockAvailable(requestedSlot uint64) bool
	Fetch(ctx context.Context, blkNum uint64) (b *pbbstream.Block, skipped bool, err error)
}
