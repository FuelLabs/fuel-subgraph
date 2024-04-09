package firehose

import (
	"context"
	"fmt"
	"strconv"
	"strings"

	"github.com/spf13/cobra"
	firecore "github.com/streamingfast/firehose-core"
	"github.com/streamingfast/jsonpb"
	"github.com/streamingfast/logging"
	pbfirehose "github.com/streamingfast/pbgo/sf/firehose/v2"
	"go.uber.org/zap"
)

// You should add your custom 'transforms' flags to this command in your init(), then parse them in transformsSetter
func NewToolsFirehoseSingleBlockClientCmd[B firecore.Block](chain *firecore.Chain[B], zlog *zap.Logger, tracer logging.Tracer) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "firehose-single-block-client {endpoint} {block_num|block_num:block_id|cursor}",
		Short: "fetch a single block from firehose and print as JSON",
		Args:  cobra.ExactArgs(2),
		RunE:  getFirehoseSingleBlockClientE(chain, zlog, tracer),
		Example: firecore.ExamplePrefixed(chain, "tools ", `
			firehose-single-block-client --compression=gzip my.firehose.endpoint:443 2344:0x32d8e8d98a798da98d6as9d69899as86s9898d8ss8d87
		`),
	}

	addFirehoseFetchClientFlagsToSet(cmd.Flags(), chain)

	return cmd
}

func getFirehoseSingleBlockClientE[B firecore.Block](chain *firecore.Chain[B], zlog *zap.Logger, tracer logging.Tracer) func(cmd *cobra.Command, args []string) error {
	return func(cmd *cobra.Command, args []string) error {
		ctx := context.Background()

		endpoint := args[0]
		firehoseClient, connClose, requestInfo, err := getFirehoseFetchClientFromCmd(cmd, zlog, endpoint, chain)
		if err != nil {
			return err
		}
		defer connClose()

		req := &pbfirehose.SingleBlockRequest{}

		ref := args[1]
		if num, err := strconv.ParseUint(ref, 10, 64); err == nil {
			req.Reference = &pbfirehose.SingleBlockRequest_BlockNumber_{
				BlockNumber: &pbfirehose.SingleBlockRequest_BlockNumber{
					Num: num,
				},
			}
		} else if parts := strings.Split(ref, ":"); len(parts) == 2 {
			num, err := strconv.ParseUint(parts[0], 10, 64)
			if err != nil {
				return fmt.Errorf("invalid block reference, cannot decode first part as block_num: %s, %w", ref, err)
			}
			req.Reference = &pbfirehose.SingleBlockRequest_BlockHashAndNumber_{
				BlockHashAndNumber: &pbfirehose.SingleBlockRequest_BlockHashAndNumber{
					Num:  num,
					Hash: parts[1],
				},
			}

		} else {
			req.Reference = &pbfirehose.SingleBlockRequest_Cursor_{
				Cursor: &pbfirehose.SingleBlockRequest_Cursor{
					Cursor: ref,
				},
			}
		}

		resp, err := firehoseClient.Block(ctx, req, requestInfo.GRPCCallOpts...)
		if err != nil {
			return err
		}

		line, err := jsonpb.MarshalToString(resp)
		if err != nil {
			return err
		}
		fmt.Println(line)
		return nil
	}
}
