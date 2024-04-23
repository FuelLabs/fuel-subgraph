// Copyright 2021 dfuse Platform Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package compare

import (
	"context"
	"fmt"
	"io"
	"reflect"
	"strconv"
	"sync"

	jd "github.com/josephburnett/jd/lib"
	"github.com/spf13/cobra"
	"github.com/streamingfast/bstream"
	"github.com/streamingfast/cli"
	"github.com/streamingfast/cli/sflags"
	"github.com/streamingfast/dstore"
	firecore "github.com/streamingfast/firehose-core"
	"github.com/streamingfast/firehose-core/cmd/tools/check"
	"github.com/streamingfast/firehose-core/json"
	fcproto "github.com/streamingfast/firehose-core/proto"
	"github.com/streamingfast/firehose-core/types"
	"go.uber.org/multierr"
	"google.golang.org/protobuf/proto"
	"google.golang.org/protobuf/types/dynamicpb"
)

type BlockDifferences struct {
	BlockNumber uint64
	Differences []string
}

func NewToolsCompareBlocksCmd[B firecore.Block](chain *firecore.Chain[B]) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "compare-blocks <reference_blocks_store> <current_blocks_store> [<block_range>]",
		Short: "Checks for any differences between two block stores between a specified range. (To compare the likeness of two block ranges, for example)",
		Long: cli.Dedent(`
			The 'compare-blocks' takes in two paths to stores of merged blocks and a range specifying the blocks you
			want to compare, written as: '<start>:<finish>'. It will output the status of the likeness of every
			100,000 blocks, on completion, or on encountering a difference. Increments that contain a difference will
			be communicated as well as the blocks within that contain differences. Increments that do not have any
			differences will be outputted as identical.

			After passing through the blocks, it will output instructions on how to locate a specific difference
			based on the blocks that were given. This is done by applying the '--diff' flag before your args.

			Commands inputted with '--diff' will display the blocks that have differences, as well as the
			difference.
		`),
		Args: cobra.ExactArgs(3),
		RunE: runCompareBlocksE(chain),
		Example: firecore.ExamplePrefixed(chain, "tools compare-blocks", `
			# Run over full block range
			reference_store/ current_store/ 0:16000000

			# Run over specific block range, displaying differences in blocks
			--diff reference_store/ current_store/ 100:200
		`),
	}

	flags := cmd.PersistentFlags()
	flags.Bool("diff", false, "When activated, difference is displayed for each block with a difference")
	flags.String("bytes-encoding", "hex", "Encoding for bytes fields, either 'hex' or 'base58'")
	flags.Bool("include-unknown-fields", false, "When activated, the 'unknown fields' in the protobuf message will also be compared. These would not generate any difference when unmarshalled with the current protobuf definition.")
	flags.StringSlice("proto-paths", []string{""}, "Paths to proto files to use for dynamic decoding of blocks")

	return cmd
}

func runCompareBlocksE[B firecore.Block](chain *firecore.Chain[B]) firecore.CommandExecutor {

	return func(cmd *cobra.Command, args []string) error {
		displayDiff := sflags.MustGetBool(cmd, "diff")
		includeUnknownFields := sflags.MustGetBool(cmd, "include-unknown-fields")
		protoPaths := sflags.MustGetStringSlice(cmd, "proto-paths")
		bytesEncoding := sflags.MustGetString(cmd, "bytes-encoding")
		segmentSize := uint64(100000)
		warnAboutExtraBlocks := sync.Once{}
		ctx := cmd.Context()
		blockRange, err := types.GetBlockRangeFromArg(args[2])
		if err != nil {
			return fmt.Errorf("parsing range: %w", err)
		}

		if !blockRange.IsResolved() {
			return fmt.Errorf("invalid block range, you must provide a closed range fully resolved (no negative value)")
		}

		stopBlock := blockRange.GetStopBlockOr(firecore.MaxUint64)

		// Create stores
		storeReference, err := dstore.NewDBinStore(args[0])
		if err != nil {
			return fmt.Errorf("unable to create store at path %q: %w", args[0], err)
		}
		storeCurrent, err := dstore.NewDBinStore(args[1])
		if err != nil {
			return fmt.Errorf("unable to create store at path %q: %w", args[1], err)
		}

		segments, err := blockRange.Split(segmentSize, types.EndBoundaryExclusive)
		if err != nil {
			return fmt.Errorf("unable to split blockrage in segments: %w", err)
		}
		processState := &state{
			segments: segments,
		}

		registry, err := fcproto.NewRegistry(nil, protoPaths...)
		if err != nil {
			return fmt.Errorf("creating registry: %w", err)
		}

		sanitizer := chain.Tools.GetSanitizeBlockForCompare()

		err = storeReference.Walk(ctx, check.WalkBlockPrefix(blockRange, 100), func(filename string) (err error) {
			fileStartBlock, err := strconv.Atoi(filename)
			if err != nil {
				return fmt.Errorf("parsing filename: %w", err)
			}

			// If reached end of range
			if stopBlock <= uint64(fileStartBlock) {
				return dstore.StopIteration
			}

			if blockRange.Contains(uint64(fileStartBlock), types.EndBoundaryExclusive) {
				var wg sync.WaitGroup
				var bundleErrLock sync.Mutex
				var bundleReadErr error
				var referenceBlockHashes []string
				var referenceBlocks map[string]*dynamicpb.Message
				var referenceBlocksNum map[string]uint64
				var currentBlocks map[string]*dynamicpb.Message

				wg.Add(1)
				go func() {
					defer wg.Done()
					referenceBlockHashes, referenceBlocks, referenceBlocksNum, err = readBundle(
						ctx,
						filename,
						storeReference,
						uint64(fileStartBlock),
						stopBlock,
						&warnAboutExtraBlocks,
						sanitizer,
						registry,
					)
					if err != nil {
						bundleErrLock.Lock()
						bundleReadErr = multierr.Append(bundleReadErr, err)
						bundleErrLock.Unlock()
					}
				}()

				wg.Add(1)
				go func() {
					defer wg.Done()
					_, currentBlocks, _, err = readBundle(ctx,
						filename,
						storeCurrent,
						uint64(fileStartBlock),
						stopBlock,
						&warnAboutExtraBlocks,
						sanitizer,
						registry,
					)
					if err != nil {
						bundleErrLock.Lock()
						bundleReadErr = multierr.Append(bundleReadErr, err)
						bundleErrLock.Unlock()
					}
				}()
				wg.Wait()
				if bundleReadErr != nil {
					return fmt.Errorf("reading bundles: %w", bundleReadErr)
				}

				outLock := sync.Mutex{}
				for _, referenceBlockHash := range referenceBlockHashes {
					wg.Add(1)
					go func(hash string) {
						defer wg.Done()
						referenceBlock := referenceBlocks[hash]
						currentBlock, existsInCurrent := currentBlocks[hash]
						referenceBlockNum := referenceBlocksNum[hash]

						var isDifferent bool
						if existsInCurrent {
							var differences []string
							differences = Compare(referenceBlock, currentBlock, includeUnknownFields, registry, bytesEncoding)

							isDifferent = len(differences) > 0

							if isDifferent {
								outLock.Lock()
								fmt.Printf("- Block %d is different\n", referenceBlockNum)
								if displayDiff {
									for _, diff := range differences {
										fmt.Println("  · ", diff)
									}
								}
								outLock.Unlock()
							}
						}
						processState.process(referenceBlockNum, isDifferent, !existsInCurrent)
					}(referenceBlockHash)
					wg.Wait()
				}
			}
			return nil
		})
		if err != nil {
			return fmt.Errorf("walking files: %w", err)
		}
		processState.print()

		return nil
	}
}

func readBundle(
	ctx context.Context,
	filename string,
	store dstore.Store,
	fileStartBlock,
	stopBlock uint64,
	warnAboutExtraBlocks *sync.Once,
	sanitizer firecore.SanitizeBlockForCompareFunc,
	registry *fcproto.Registry,
) ([]string, map[string]*dynamicpb.Message, map[string]uint64, error) {
	fileReader, err := store.OpenObject(ctx, filename)

	if err != nil {
		return nil, nil, nil, fmt.Errorf("creating reader: %w", err)
	}

	blockReader, err := bstream.NewDBinBlockReader(fileReader)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("creating block reader: %w", err)
	}

	var blockHashes []string
	blocksMap := make(map[string]*dynamicpb.Message)
	blockNumMap := make(map[string]uint64)
	for {
		curBlock, err := blockReader.Read()
		if err == io.EOF {
			break
		}
		if err != nil {
			return nil, nil, nil, fmt.Errorf("reading blocks: %w", err)
		}
		if curBlock.Number >= stopBlock {
			break
		}
		if curBlock.Number < fileStartBlock {
			warnAboutExtraBlocks.Do(func() {
				fmt.Printf("Warn: Bundle file %s contains block %d, preceding its start_block. This 'feature' is not used anymore and extra blocks like this one will be ignored during compare\n", store.ObjectURL(filename), curBlock.Number)
			})
			continue
		}

		if sanitizer != nil {
			curBlock = sanitizer(curBlock)
		}

		curBlockPB, err := registry.Unmarshal(curBlock.Payload)

		if err != nil {
			return nil, nil, nil, fmt.Errorf("unmarshalling block: %w", err)
		}
		blockHashes = append(blockHashes, curBlock.Id)
		blockNumMap[curBlock.Id] = curBlock.Number
		blocksMap[curBlock.Id] = curBlockPB
	}

	return blockHashes, blocksMap, blockNumMap, nil
}

type state struct {
	segments                   []types.BlockRange
	currentSegmentIdx          int
	blocksCountedInThisSegment int
	differencesFound           int
	missingBlocks              int
	totalBlocksCounted         int
}

func (s *state) process(blockNum uint64, isDifferent bool, isMissing bool) {
	if !s.segments[s.currentSegmentIdx].Contains(blockNum, types.EndBoundaryExclusive) { // moving forward
		s.print()
		for i := s.currentSegmentIdx; i < len(s.segments); i++ {
			if s.segments[i].Contains(blockNum, types.EndBoundaryExclusive) {
				s.currentSegmentIdx = i
				s.totalBlocksCounted += s.blocksCountedInThisSegment
				s.differencesFound = 0
				s.missingBlocks = 0
				s.blocksCountedInThisSegment = 0
			}
		}
	}

	s.totalBlocksCounted++
	if isMissing {
		s.missingBlocks++
	} else if isDifferent {
		s.differencesFound++
	}

}

func (s *state) print() {
	endBlock := fmt.Sprintf("%d", s.segments[s.currentSegmentIdx].GetStopBlockOr(firecore.MaxUint64))

	if s.totalBlocksCounted == 0 {
		fmt.Printf("✖ No blocks were found at all for segment %d - %s\n", s.segments[s.currentSegmentIdx].Start, endBlock)
		return
	}

	if s.differencesFound == 0 && s.missingBlocks == 0 {
		fmt.Printf("✓ Segment %d - %s has no differences (%d blocks counted)\n", s.segments[s.currentSegmentIdx].Start, endBlock, s.totalBlocksCounted)
		return
	}

	if s.differencesFound == 0 && s.missingBlocks == 0 {
		fmt.Printf("✓~ Segment %d - %s has no differences but does have %d missing blocks (%d blocks counted)\n", s.segments[s.currentSegmentIdx].Start, endBlock, s.missingBlocks, s.totalBlocksCounted)
		return
	}

	fmt.Printf("✖ Segment %d - %s has %d different blocks and %d missing blocks (%d blocks counted)\n", s.segments[s.currentSegmentIdx].Start, endBlock, s.differencesFound, s.missingBlocks, s.totalBlocksCounted)
}

func Compare(reference proto.Message, current proto.Message, includeUnknownFields bool, registry *fcproto.Registry, bytesEncoding string) (differences []string) {
	if reference == nil && current == nil {
		return nil
	}
	if reflect.TypeOf(reference).Kind() == reflect.Ptr && reference == current {
		return nil
	}

	referenceMsg := reference.ProtoReflect()
	currentMsg := current.ProtoReflect()
	if referenceMsg.IsValid() && !currentMsg.IsValid() {
		return []string{fmt.Sprintf("reference block is valid protobuf message, but current block is invalid")}
	}
	if !referenceMsg.IsValid() && currentMsg.IsValid() {
		return []string{fmt.Sprintf("reference block is invalid protobuf message, but current block is valid")}
	}

	//todo: check if there is a equals that do not compare unknown fields
	if !proto.Equal(reference, current) {
		var opts []json.MarshallerOption
		if !includeUnknownFields {
			opts = append(opts, json.WithoutUnknownFields())
		}

		if bytesEncoding == "base58" {
			opts = append(opts, json.WithBytesEncoderFunc(json.ToBase58))
		}

		encoder := json.NewMarshaller(registry, opts...)

		referenceAsJSON, err := encoder.MarshalToString(reference)
		cli.NoError(err, "marshal JSON reference")

		currentAsJSON, err := encoder.MarshalToString(current)
		cli.NoError(err, "marshal JSON current")

		r, err := jd.ReadJsonString(referenceAsJSON)
		cli.NoError(err, "read JSON reference")

		c, err := jd.ReadJsonString(currentAsJSON)
		cli.NoError(err, "read JSON current")

		if diff := r.Diff(c).Render(); diff != "" {

			differences = append(differences, diff)
		}

	}

	return differences
}
