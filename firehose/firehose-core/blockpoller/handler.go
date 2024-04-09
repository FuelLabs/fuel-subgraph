package blockpoller

import (
	"encoding/base64"
	"fmt"
	"strings"
	"sync"

	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
)

type BlockHandler interface {
	Init()
	Handle(blk *pbbstream.Block) error
}

var _ BlockHandler = (*FireBlockHandler)(nil)

type FireBlockHandler struct {
	blockTypeURL string
	init         sync.Once
}

func NewFireBlockHandler(blockTypeURL string) *FireBlockHandler {
	return &FireBlockHandler{
		blockTypeURL: clean(blockTypeURL),
	}
}

func (f *FireBlockHandler) Init() {
	fmt.Println("FIRE INIT 3.0", f.blockTypeURL)
}

func (f *FireBlockHandler) Handle(b *pbbstream.Block) error {
	typeURL := clean(b.Payload.TypeUrl)
	if typeURL != f.blockTypeURL {
		return fmt.Errorf("block type url %q does not match expected type %q", typeURL, f.blockTypeURL)
	}

	blockLine := fmt.Sprintf(
		"FIRE BLOCK %d %s %d %s %d %d %s",
		b.Number,
		b.Id,
		b.ParentNum,
		b.ParentId,
		b.LibNum,
		b.Timestamp.AsTime().UnixNano(),
		base64.StdEncoding.EncodeToString(b.Payload.Value),
	)

	fmt.Println(blockLine)
	return nil
}

func clean(in string) string {
	return strings.Replace(in, "type.googleapis.com/", "", 1)
}
