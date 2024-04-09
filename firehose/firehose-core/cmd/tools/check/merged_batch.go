package check

import (
	"context"
	"fmt"
	"io"
	"strconv"
	"strings"

	"github.com/streamingfast/bstream"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/streamingfast/dstore"
	"github.com/streamingfast/firehose-core/types"
)

type blockRef struct {
	hash string
	num  uint64
}

func (b *blockRef) reset() {
	b.hash = ""
	b.num = 0
}

func (b *blockRef) set(hash string, num uint64) {
	b.hash = hash
	b.num = num
}

func (b *blockRef) isUnset() bool {
	return b.hash == "" && b.num == 0
}

// CheckMergedBlocksBatch will write a list of base-block-numbers to a store, for merged-blocks-files that are broken or missing
// broken merged-blocks-files are the ones that contain "empty" blocks (no ID) or unlinkable blocks
// there could be false positives on unlinkable blocks, though
// output files are like this: 0000123100.broken  0000123500.missing
func CheckMergedBlocksBatch(
	ctx context.Context,
	sourceStoreURL string,
	destStoreURL string,
	fileBlockSize uint64,
	blockRange types.BlockRange,
) error {
	if !blockRange.IsResolved() {
		return fmt.Errorf("check merged blocks can only work with fully resolved range, got %s", blockRange)
	}

	expected := types.RoundToBundleStartBlock(uint64(blockRange.Start), fileBlockSize)
	fileBlockSize64 := uint64(fileBlockSize)

	blocksStore, err := dstore.NewDBinStore(sourceStoreURL)
	if err != nil {
		return err
	}
	var destStore dstore.Store
	if destStoreURL != "" {
		destStore, err = dstore.NewSimpleStore(destStoreURL)
		if err != nil {
			return err
		}
	}

	var firstFilename = fmt.Sprintf("%010d", types.RoundToBundleStartBlock(uint64(blockRange.Start), fileBlockSize))

	lastSeenBlock := &blockRef{}

	var lastFilename string
	err = blocksStore.WalkFrom(ctx, "", firstFilename, func(filename string) error {
		if strings.HasSuffix(filename, ".tmp") {
			return nil
		}
		match := numberRegex.FindStringSubmatch(filename)
		if match == nil {
			return nil
		}

		// should not happen with firstFilename, but leaving in case
		baseNum, _ := strconv.ParseUint(match[1], 10, 32)
		if baseNum+uint64(fileBlockSize)-1 < uint64(blockRange.Start) {
			return nil
		}

		if baseNum < uint64(expected) {
			return fmt.Errorf("unhandled error: found base number %d below expected %d", baseNum, expected)
		}
		for expected < baseNum {
			fmt.Printf("missing file %q\n", filename)
			if destStore != nil {
				outputFile := fmt.Sprintf("%010d.missing", expected)
				destStore.WriteObject(ctx, outputFile, strings.NewReader(""))
			}
			expected += fileBlockSize64
		}

		broken, details, err := checkMergedBlockFileBroken(ctx, blocksStore, filename, lastSeenBlock)
		if broken {
			if lastSeenBlock.isUnset() {
				fmt.Printf("found broken file %q, %s\n", filename, details)
				if destStore != nil {
					outputFile := fmt.Sprintf("%010d.broken", baseNum)
					destStore.WriteObject(ctx, outputFile, strings.NewReader(""))
				}
			} else {
				brokenSince := types.RoundToBundleStartBlock(uint64(lastSeenBlock.num+1), 100)
				for i := brokenSince; i <= baseNum; i += fileBlockSize64 {
					fmt.Printf("found broken file %q, %s\n", filename, details)
					if destStore != nil {
						outputFile := fmt.Sprintf("%010d.broken", i)
						err := destStore.WriteObject(ctx, outputFile, strings.NewReader(""))
						if err != nil {
							return fmt.Errorf("unable to write broken file %q: %w", outputFile, err)
						}
					}
				}
			}
			lastSeenBlock.reset()
		}
		lastFilename = filename

		if err != nil {
			return err
		}

		if blockRange.IsClosed() && types.RoundToBundleEndBlock(baseNum, fileBlockSize) >= *blockRange.Stop-1 {
			return dstore.StopIteration
		}
		expected = baseNum + fileBlockSize64

		return nil
	})
	fmt.Println("last file processed:", lastFilename)
	if err != nil {
		return err
	}

	return nil
}

var printCounter = 0

func checkMergedBlockFileBroken(
	ctx context.Context,
	store dstore.Store,
	filename string,
	lastSeenBlock *blockRef,
) (broken bool, details string, err error) {
	if printCounter%100 == 0 {
		fmt.Println("checking", filename, "... (printing 1/100)")
	}
	printCounter++

	reader, err := store.OpenObject(ctx, filename)
	if err != nil {
		return true, "", err
	}
	defer reader.Close()

	readerFactory, err := bstream.NewDBinBlockReader(reader)
	if err != nil {
		return true, "", err
	}

	for {
		var block *pbbstream.Block
		block, err = readerFactory.Read()

		if block == nil {
			if err == io.EOF {
				err = nil
			}
			return
		}
		if err != nil {
			return
		}

		if block.Id == "" {
			broken = true
			details = "read block with no ID"
			return
		}

		if lastSeenBlock.isUnset() {
			fakePreviousNum := block.Number
			if fakePreviousNum != 0 {
				fakePreviousNum -= 1
			}
			lastSeenBlock.set(block.ParentId, fakePreviousNum)
		}
		if block.ParentId != lastSeenBlock.hash {
			if block.Id == lastSeenBlock.hash && block.Number == lastSeenBlock.num {
				continue
			}
			details = fmt.Sprintf("broken on block %d: expecting %q, got %q", block.Number, lastSeenBlock.hash, block.ParentId)
			broken = true
			return
		}
		lastSeenBlock.set(block.Id, block.Number)
	}
}
