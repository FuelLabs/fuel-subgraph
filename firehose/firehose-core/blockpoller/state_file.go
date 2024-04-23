package blockpoller

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/bstream/forkable"
	"go.uber.org/zap"
)

type blockRef struct {
	Id  string `json:"id"`
	Num uint64 `json:"num"`
}

type blockRefWithPrev struct {
	blockRef
	PrevBlockId string `json:"previous_ref_id"`
}

func (b blockRef) String() string {
	return fmt.Sprintf("%d (%s)", b.Num, b.Id)
}

type stateFile struct {
	Lib            blockRef
	LastFiredBlock blockRefWithPrev
	Blocks         []blockRefWithPrev
}

func getState(stateStorePath string) (*stateFile, error) {
	if stateStorePath == "" {
		return nil, fmt.Errorf("no cursor store path set")
	}

	filepath := filepath.Join(stateStorePath, "cursor.json")
	file, err := os.Open(filepath)
	if err != nil {
		return nil, fmt.Errorf("unable to open cursor file %s: %w", filepath, err)
	}
	sf := stateFile{}
	decoder := json.NewDecoder(file)
	if err := decoder.Decode(&sf); err != nil {
		return nil, fmt.Errorf("feailed to decode cursor file %s: %w", filepath, err)
	}
	return &sf, nil
}

func (p *BlockPoller) saveState(blocks []*forkable.Block) error {
	p.logger.Debug("saving cursor", zap.String("state_store_path", p.stateStorePath))
	if p.stateStorePath == "" {
		return nil
	}

	lastFiredBlock := blocks[len(blocks)-1]

	sf := stateFile{
		Lib:            blockRef{p.forkDB.LIBID(), p.forkDB.LIBNum()},
		LastFiredBlock: blockRefWithPrev{blockRef{lastFiredBlock.BlockID, lastFiredBlock.BlockNum}, lastFiredBlock.PreviousBlockID},
	}

	for _, blk := range blocks {
		sf.Blocks = append(sf.Blocks, blockRefWithPrev{blockRef{blk.BlockID, blk.BlockNum}, blk.PreviousBlockID})
	}

	cnt, err := json.Marshal(sf)
	if err != nil {
		return fmt.Errorf("unable to marshal stateFile: %w", err)
	}

	err = os.MkdirAll(p.stateStorePath, os.ModePerm)
	if err != nil {
		return fmt.Errorf("making state store path: %w", err)
	}
	fpath := filepath.Join(p.stateStorePath, "cursor.json")

	if err := os.WriteFile(fpath, cnt, 0666); err != nil {
		return fmt.Errorf("unable to open cursor file %s: %w", fpath, err)
	}

	p.logger.Info("saved cursor",
		zap.Reflect("filepath", fpath),
		zap.Stringer("last_fired_block", sf.LastFiredBlock),
		zap.Stringer("lib", sf.Lib),
		zap.Int("block_count", len(sf.Blocks)),
	)
	return nil
}

func initState(resolvedStartBlock bstream.BlockRef, stateStorePath string, ignoreCursor bool, logger *zap.Logger) (*forkable.ForkDB, bstream.BlockRef, error) {
	forkDB := forkable.NewForkDB(forkable.ForkDBWithLogger(logger))

	useStartBlockFunc := func() (*forkable.ForkDB, bstream.BlockRef, error) {
		forkDB.InitLIB(resolvedStartBlock)
		return forkDB, resolvedStartBlock, nil
	}

	if ignoreCursor {
		logger.Info("ignorign cursor",
			zap.Stringer("start_block", resolvedStartBlock),
			zap.Stringer("lib", resolvedStartBlock),
		)
		return useStartBlockFunc()
	}

	sf, err := getState(stateStorePath)
	if err != nil {
		logger.Warn("unable to load cursor file, initializing a new forkdb",
			zap.Stringer("start_block", resolvedStartBlock),
			zap.Stringer("lib", resolvedStartBlock),
			zap.Error(err),
		)
		return useStartBlockFunc()
	}

	forkDB.InitLIB(bstream.NewBlockRef(sf.Lib.Id, sf.Lib.Num))

	for _, blk := range sf.Blocks {
		b := &block{
			Block: &pbbstream.Block{
				Number:   blk.Num,
				Id:       blk.Id,
				ParentId: blk.PrevBlockId,
			},
			fired: true,
		}
		forkDB.AddLink(bstream.NewBlockRef(blk.Id, blk.Num), blk.PrevBlockId, b)
	}

	logger.Info("loaded cursor",
		zap.Stringer("start_block", sf.LastFiredBlock),
		zap.Stringer("lib", sf.Lib),
		zap.Int("block_count", len(sf.Blocks)),
	)

	return forkDB, bstream.NewBlockRef(sf.LastFiredBlock.Id, sf.LastFiredBlock.Num), nil
}
