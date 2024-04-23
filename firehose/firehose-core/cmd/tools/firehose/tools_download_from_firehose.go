package firehose

import (
	"context"
	"fmt"

	"io"
	"strconv"
	"time"

	"github.com/spf13/cobra"
	"github.com/streamingfast/bstream"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/streamingfast/dstore"
	firecore "github.com/streamingfast/firehose-core"
	pbfirehose "github.com/streamingfast/pbgo/sf/firehose/v2"
	"go.uber.org/zap"
	"google.golang.org/protobuf/proto"
	"google.golang.org/protobuf/types/known/anypb"
)

func NewToolsDownloadFromFirehoseCmd[B firecore.Block](chain *firecore.Chain[B], zlog *zap.Logger) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "download-from-firehose <endpoint> <range> <destination>",
		Short: "Download blocks from Firehose and save them to merged-blocks",
		Args:  cobra.ExactArgs(4),
		RunE:  createToolsDownloadFromFirehoseE(chain, zlog),
		Example: firecore.ExamplePrefixed(chain, "tools download-from-firehose", `
			# Adjust <url> based on your actual network
			mainnet.eth.streamingfast.io:443 1000:2000 ./output_dir
		`),
	}

	addFirehoseStreamClientFlagsToSet(cmd.Flags(), chain)

	return cmd
}

func createToolsDownloadFromFirehoseE[B firecore.Block](chain *firecore.Chain[B], zlog *zap.Logger) func(cmd *cobra.Command, args []string) error {
	return func(cmd *cobra.Command, args []string) error {
		ctx := context.Background()

		if _, ok := chain.BlockFactory().(*pbbstream.Block); ok {
			//todo: fix this with buf registry
			return fmt.Errorf("this tool only works with blocks that are not of type *pbbstream.Block")
		}

		endpoint := args[0]
		startBlock, err := strconv.ParseUint(args[1], 10, 64)
		if err != nil {
			return fmt.Errorf("parsing start block num: %w", err)
		}
		stopBlock, err := strconv.ParseUint(args[2], 10, 64)
		if err != nil {
			return fmt.Errorf("parsing stop block num: %w", err)
		}
		destFolder := args[3]

		firehoseClient, connClose, requestInfo, err := getFirehoseStreamClientFromCmd(cmd, zlog, endpoint, chain)
		if err != nil {
			return err
		}
		defer connClose()

		var retryDelay = time.Second * 4

		store, err := dstore.NewDBinStore(destFolder)
		if err != nil {
			return err
		}

		mergeWriter := &firecore.MergedBlocksWriter{
			Store:      store,
			TweakBlock: func(b *pbbstream.Block) (*pbbstream.Block, error) { return b, nil },
			Logger:     zlog,
		}

		approximateLIBWarningIssued := false
		var lastBlockID string
		var lastBlockNum uint64
		for {

			request := &pbfirehose.Request{
				StartBlockNum:   int64(startBlock),
				StopBlockNum:    stopBlock,
				FinalBlocksOnly: true,
				Cursor:          requestInfo.Cursor,
			}

			stream, err := firehoseClient.Blocks(ctx, request, requestInfo.GRPCCallOpts...)
			if err != nil {
				return fmt.Errorf("unable to start blocks stream: %w", err)
			}

			for {
				response, err := stream.Recv()
				if err != nil {
					if err == io.EOF {
						return nil
					}

					zlog.Error("stream encountered a remote error, going to retry",
						zap.Duration("retry_delay", retryDelay),
						zap.Error(err),
					)
					<-time.After(retryDelay)
					break
				}

				block := chain.BlockFactory()
				if err := anypb.UnmarshalTo(response.Block, block, proto.UnmarshalOptions{}); err != nil {
					return fmt.Errorf("unmarshal response block: %w", err)
				}

				if _, ok := block.(firecore.BlockLIBNumDerivable); !ok {
					// We must wrap the block in a BlockEnveloppe and "provide" the LIB number as itself minus 1 since
					// there is nothing we can do more here to obtain the value sadly. For chain where the LIB can be
					// derived from the Block itself, this code does **not** run (so it will have the correct value)
					if !approximateLIBWarningIssued {
						approximateLIBWarningIssued = true
						zlog.Warn("LIB number is approximated, it is not provided by the chain's Block model so we msut set it to block number minus 1 (which is kinda ok because only final blocks are retrieved in this download tool)")
					}

					number := block.GetFirehoseBlockNumber()
					libNum := number - 1
					if number <= bstream.GetProtocolFirstStreamableBlock {
						libNum = number
					}

					block = firecore.BlockEnveloppe{
						Block:  block,
						LIBNum: libNum,
					}
				}

				blk, err := chain.BlockEncoder.Encode(block)
				if err != nil {
					return fmt.Errorf("error decoding response to bstream block: %w", err)
				}
				if lastBlockID != "" && blk.ParentId != lastBlockID {
					return fmt.Errorf("got an invalid sequence of blocks: block %q has previousId %s, previous block %d had ID %q, this endpoint is serving blocks out of order", blk.String(), blk.ParentId, lastBlockNum, lastBlockID)
				}
				lastBlockID = blk.Id
				lastBlockNum = blk.Number

				if err := mergeWriter.ProcessBlock(blk, nil); err != nil {
					return fmt.Errorf("write to blockwriter: %w", err)
				}
			}
		}
	}
}
