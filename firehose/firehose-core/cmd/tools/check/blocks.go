package check

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"regexp"
	"strconv"

	"github.com/streamingfast/bstream"
	"github.com/streamingfast/bstream/forkable"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/streamingfast/dstore"
	firecore "github.com/streamingfast/firehose-core"
	print2 "github.com/streamingfast/firehose-core/cmd/tools/print"
	"github.com/streamingfast/firehose-core/types"
	"go.uber.org/zap"
)

var numberRegex = regexp.MustCompile(`(\d{10})`)

type PrintDetails uint8

const (
	PrintNoDetails PrintDetails = iota
	PrintStats
	PrintFull
)

func CheckMergedBlocks[B firecore.Block](ctx context.Context, chain *firecore.Chain[B], logger *zap.Logger, storeURL string, fileBlockSize uint64, blockRange types.BlockRange, printDetails PrintDetails) error {
	readAllBlocks := printDetails != PrintNoDetails
	fmt.Printf("Checking block holes on %s\n", storeURL)
	if readAllBlocks {
		fmt.Println("Detailed printing requested: All block files will be read and checked for continuity. This may take a while...")
	}

	var expected uint64
	var count int
	var highestBlockSeen uint64
	lowestBlockSeen := firecore.MaxUint64

	holeFound := false
	expected = types.RoundToBundleStartBlock(uint64(blockRange.Start), fileBlockSize)
	currentStartBlk := uint64(blockRange.Start)

	blocksStore, err := dstore.NewDBinStore(storeURL)
	if err != nil {
		return err
	}

	walkPrefix := WalkBlockPrefix(blockRange, fileBlockSize)

	tfdb := &trackedForkDB{
		fdb: forkable.NewForkDB(),
	}

	logger.Debug("walking merged blocks", zap.Stringer("block_range", blockRange), zap.String("walk_prefix", walkPrefix))
	err = blocksStore.Walk(ctx, walkPrefix, func(filename string) error {
		match := numberRegex.FindStringSubmatch(filename)
		if match == nil {
			return nil
		}

		logger.Debug("received merged blocks", zap.String("filename", filename))

		count++
		baseNum, _ := strconv.ParseUint(match[1], 10, 32)
		if baseNum+uint64(fileBlockSize)-1 < uint64(blockRange.Start) {
			logger.Debug("base num lower then block range start, quitting", zap.Uint64("base_num", baseNum), zap.Int64("starting_at", blockRange.Start))
			return nil
		}

		if baseNum != expected {
			// There is no previous valid block range if we are at the ever first seen file
			if count > 1 {
				fmt.Printf("✅ Range %s\n", types.NewClosedRange(int64(currentStartBlk), uint64(types.RoundToBundleEndBlock(expected-fileBlockSize, fileBlockSize))))
			}

			// Otherwise, we do not follow last seen element (previous is `100 - 199` but we are `299 - 300`)
			missingRange := types.NewClosedRange(int64(expected), types.RoundToBundleEndBlock(baseNum-fileBlockSize, fileBlockSize))
			fmt.Printf("❌ Range %s (Missing, [%s])\n", missingRange, missingRange.ReprocRange())
			currentStartBlk = baseNum

			holeFound = true
		}
		expected = baseNum + fileBlockSize

		if readAllBlocks {
			lowestBlockSegment, highestBlockSegment := validateBlockSegment(ctx, chain, blocksStore, filename, fileBlockSize, blockRange, printDetails, tfdb)
			if lowestBlockSegment < lowestBlockSeen {
				lowestBlockSeen = lowestBlockSegment
			}
			if highestBlockSegment > highestBlockSeen {
				highestBlockSeen = highestBlockSegment
			}
		} else {
			if baseNum < lowestBlockSeen {
				lowestBlockSeen = baseNum
			}
			if baseNum+fileBlockSize > highestBlockSeen {
				highestBlockSeen = baseNum + fileBlockSize
			}
		}

		if count%10000 == 0 {
			fmt.Printf("✅ Range %s\n", types.NewClosedRange(int64(currentStartBlk), types.RoundToBundleEndBlock(baseNum, fileBlockSize)))
			currentStartBlk = baseNum + fileBlockSize
		}

		if blockRange.IsClosed() && types.RoundToBundleEndBlock(baseNum, fileBlockSize) >= *blockRange.Stop-1 {
			return dstore.StopIteration
		}

		return nil
	})

	if err != nil {
		return err
	}

	logger.Debug("checking incomplete range",
		zap.Stringer("range", blockRange),
		zap.Bool("range_unbounded", blockRange.IsOpen()),
		zap.Uint64("lowest_block_seen", lowestBlockSeen),
		zap.Uint64("highest_block_seen", highestBlockSeen),
	)
	if tfdb.lastLinkedBlock != nil && tfdb.lastLinkedBlock.Number < highestBlockSeen {
		fmt.Printf("🔶 Range %s has issues with forks, last linkable block number: %d\n", types.NewClosedRange(int64(currentStartBlk), uint64(highestBlockSeen)), tfdb.lastLinkedBlock.Number)
	} else {
		fmt.Printf("✅ Range %s\n", types.NewClosedRange(int64(currentStartBlk), uint64(highestBlockSeen)))
	}

	fmt.Println()
	fmt.Println("Summary:")

	if blockRange.IsClosed() &&
		(highestBlockSeen < uint64(*blockRange.Stop-1) ||
			(lowestBlockSeen > uint64(blockRange.Start) && lowestBlockSeen > bstream.GetProtocolFirstStreamableBlock)) {
		fmt.Printf("> 🔶 Incomplete range %s, started at block %s and stopped at block: %s\n", blockRange, types.PrettyBlockNum(lowestBlockSeen), types.PrettyBlockNum(highestBlockSeen))
	}

	if holeFound {
		fmt.Printf("> 🆘 Holes found!\n")
	} else {
		fmt.Printf("> 🆗 No hole found\n")
	}

	return nil
}

type trackedForkDB struct {
	fdb                    *forkable.ForkDB
	firstUnlinkableBlock   *pbbstream.Block
	lastLinkedBlock        *pbbstream.Block
	unlinkableSegmentCount int
}

func validateBlockSegment[B firecore.Block](
	ctx context.Context,
	chain *firecore.Chain[B],
	store dstore.Store,
	segment string,
	fileBlockSize uint64,
	blockRange types.BlockRange,
	printDetails PrintDetails,
	tfdb *trackedForkDB,
) (lowestBlockSeen, highestBlockSeen uint64) {
	lowestBlockSeen = firecore.MaxUint64
	reader, err := store.OpenObject(ctx, segment)
	if err != nil {
		fmt.Printf("❌ Unable to read blocks segment %s: %s\n", segment, err)
		return
	}
	defer reader.Close()

	readerFactory, err := bstream.NewDBinBlockReader(reader)
	if err != nil {
		fmt.Printf("❌ Unable to read blocks segment %s: %s\n", segment, err)
		return
	}

	seenBlockCount := 0
	for {
		block, err := readerFactory.Read()
		if block != nil {
			if block.Number < uint64(blockRange.Start) {
				continue
			}

			if blockRange.IsClosed() && block.Number > *blockRange.Stop {
				return
			}

			if block.Number < lowestBlockSeen {
				lowestBlockSeen = block.Number
			}
			if block.Number > highestBlockSeen {
				highestBlockSeen = block.Number
			}

			if !tfdb.fdb.HasLIB() {
				tfdb.fdb.InitLIB(block.AsRef())
			}

			tfdb.fdb.AddLink(block.AsRef(), block.ParentId, nil)
			revSeg, _ := tfdb.fdb.ReversibleSegment(block.AsRef())
			if revSeg == nil {
				tfdb.unlinkableSegmentCount++
				if tfdb.firstUnlinkableBlock == nil {
					tfdb.firstUnlinkableBlock = block
				}

				// TODO: this print should be under a 'check forkable' flag?
				fmt.Printf("🔶 Block #%d is not linkable at this point\n", block.Number)

				if tfdb.unlinkableSegmentCount > 99 && tfdb.unlinkableSegmentCount%100 == 0 {
					// TODO: this print should be under a 'check forkable' flag?
					fmt.Printf("❌ Large gap of %d unlinkable blocks found in chain. Last linked block: %d, first Unlinkable block: %d. \n", tfdb.unlinkableSegmentCount, tfdb.lastLinkedBlock.Number, tfdb.firstUnlinkableBlock.Number)
				}
			} else {
				tfdb.lastLinkedBlock = block
				tfdb.unlinkableSegmentCount = 0
				tfdb.firstUnlinkableBlock = nil
				tfdb.fdb.SetLIB(block.AsRef(), block.LibNum)
				if tfdb.fdb.HasLIB() {
					tfdb.fdb.PurgeBeforeLIB(0)
				}
			}
			seenBlockCount++

			if printDetails == PrintStats {
				err := print2.PrintBStreamBlock(block, false, os.Stdout)
				if err != nil {
					fmt.Printf("❌ Unable to print block %s: %s\n", block.AsRef(), err)
					continue
				}
			}

			if printDetails == PrintFull {
				var b = chain.BlockFactory()

				if _, ok := b.(*pbbstream.Block); ok {
					//todo: implements when buf registry available ...
					panic("printing full block is not supported for pbbstream.Block")
				}

				if err := block.Payload.UnmarshalTo(b); err != nil {
					fmt.Printf("❌ Unable unmarshall block %s: %s\n", block.AsRef(), err)
					break
				}

				out, err := json.MarshalIndent(b, "", "  ")

				if err != nil {
					fmt.Printf("❌ Unable to print full block %s: %s\n", block.AsRef(), err)
					continue
				}

				fmt.Println(string(out))
			}

			continue
		}

		if block == nil && err == io.EOF {
			if seenBlockCount < expectedBlockCount(segment, fileBlockSize) {
				fmt.Printf("🔶 Segment %s contained only %d blocks (< 100), this can happen on some chains\n", segment, seenBlockCount)
			}

			return
		}

		if err != nil {
			fmt.Printf("❌ Unable to read all blocks from segment %s after reading %d blocks: %s\n", segment, seenBlockCount, err)
			return
		}
	}
	return
}

func WalkBlockPrefix(blockRange types.BlockRange, fileBlockSize uint64) string {
	if blockRange.IsOpen() {
		return ""
	}

	startString := fmt.Sprintf("%010d", types.RoundToBundleStartBlock(uint64(blockRange.Start), fileBlockSize))
	endString := fmt.Sprintf("%010d", types.RoundToBundleEndBlock(*blockRange.Stop-1, fileBlockSize)+1)

	offset := 0
	for i := 0; i < len(startString); i++ {
		if startString[i] != endString[i] {
			return string(startString[0:i])
		}

		offset++
	}

	// At this point, the two strings are equal, to return the string
	return startString
}

func expectedBlockCount(segment string, fileBlockSize uint64) int {
	if segment == "0000000000" {
		return int(fileBlockSize - bstream.GetProtocolFirstStreamableBlock)
	}

	return int(fileBlockSize)
}
