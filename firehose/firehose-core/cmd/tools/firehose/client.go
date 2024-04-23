package firehose

import (
	"context"
	"fmt"
	"io"

	"github.com/spf13/cobra"
	"github.com/streamingfast/cli/sflags"
	firecore "github.com/streamingfast/firehose-core"
	"github.com/streamingfast/firehose-core/cmd/tools/print"
	"github.com/streamingfast/firehose-core/types"
	pbfirehose "github.com/streamingfast/pbgo/sf/firehose/v2"
	"go.uber.org/zap"
)

func NewToolsFirehoseClientCmd[B firecore.Block](chain *firecore.Chain[B], logger *zap.Logger) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "firehose-client <endpoint> <range>",
		Short: "Connects to a Firehose endpoint over gRPC and print block stream as JSON to terminal",
		Args:  cobra.ExactArgs(2),
		RunE:  getFirehoseClientE(chain, logger),
	}

	addFirehoseStreamClientFlagsToSet(cmd.Flags(), chain)

	cmd.Flags().StringSlice("proto-paths", []string{""}, "Paths to proto files to use for dynamic decoding of blocks")
	cmd.Flags().Bool("final-blocks-only", false, "Only ask for final blocks")
	cmd.Flags().Bool("print-cursor-only", false, "Skip block decoding, only print the step cursor (useful for performance testing)")
	cmd.Flags().String("bytes-encoding", "hex", "Encoding for bytes fields, either 'hex' or 'base58'")

	return cmd
}

type respChan struct {
	ch chan string
}

func getFirehoseClientE[B firecore.Block](chain *firecore.Chain[B], rootLog *zap.Logger) func(cmd *cobra.Command, args []string) error {
	return func(cmd *cobra.Command, args []string) error {
		ctx := context.Background()

		firehoseClient, connClose, requestInfo, err := getFirehoseStreamClientFromCmd(cmd, rootLog, args[0], chain)
		if err != nil {
			return err
		}
		defer connClose()

		blockRange, err := types.GetBlockRangeFromArg(args[1])
		if err != nil {
			return fmt.Errorf("invalid range %q: %w", args[1], err)
		}

		printCursorOnly := sflags.MustGetBool(cmd, "print-cursor-only")

		request := &pbfirehose.Request{
			StartBlockNum:   blockRange.Start,
			StopBlockNum:    blockRange.GetStopBlockOr(0),
			Transforms:      requestInfo.Transforms,
			FinalBlocksOnly: requestInfo.FinalBlocksOnly,
			Cursor:          requestInfo.Cursor,
		}

		stream, err := firehoseClient.Blocks(ctx, request, requestInfo.GRPCCallOpts...)
		if err != nil {
			return fmt.Errorf("unable to start blocks stream: %w", err)
		}

		meta, err := stream.Header()
		if err != nil {
			rootLog.Warn("cannot read header")
		} else {
			if hosts := meta.Get("hostname"); len(hosts) != 0 {
				rootLog = rootLog.With(zap.String("remote_hostname", hosts[0]))
			}
		}
		rootLog.Info("connected")

		resps := make(chan *respChan, 10)
		allDone := make(chan bool)

		if !printCursorOnly {
			// print the responses linearly
			go func() {
				for resp := range resps {
					line := <-resp.ch
					fmt.Println(line)
				}
				close(allDone)
			}()
		}

		jencoder, err := print.SetupJsonMarshaller(cmd, chain.BlockFactory().ProtoReflect().Descriptor().ParentFile())
		if err != nil {
			return fmt.Errorf("unable to create json encoder: %w", err)
		}

		for {
			response, err := stream.Recv()
			if err != nil {
				if err == io.EOF {
					break
				}
				return fmt.Errorf("stream error while receiving: %w", err)
			}

			if printCursorOnly {
				fmt.Printf("%s - %s\n", response.Step.String(), response.Cursor)
				continue
			}

			resp := &respChan{
				ch: make(chan string),
			}
			resps <- resp

			// async process the response
			go func() {
				line, err := jencoder.MarshalToString(response)
				if err != nil {
					rootLog.Error("marshalling to string", zap.Error(err))
				}

				resp.ch <- line
			}()
		}
		if printCursorOnly {
			return nil
		}

		close(resps)
		<-allDone
		return nil
	}
}
