package firehose

import (
	"fmt"
	"os"

	"github.com/mostynb/go-grpc-compression/zstd"
	"github.com/spf13/cobra"
	"github.com/spf13/pflag"
	"github.com/streamingfast/cli/sflags"
	firecore "github.com/streamingfast/firehose-core"
	"github.com/streamingfast/firehose-core/firehose/client"
	pbfirehose "github.com/streamingfast/pbgo/sf/firehose/v2"
	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/encoding/gzip"
	"google.golang.org/protobuf/types/known/anypb"
)

type firehoseRequestInfo struct {
	GRPCCallOpts    []grpc.CallOption
	Cursor          string
	FinalBlocksOnly bool
	Transforms      []*anypb.Any
}

func getFirehoseFetchClientFromCmd[B firecore.Block](cmd *cobra.Command, logger *zap.Logger, endpoint string, chain *firecore.Chain[B]) (
	firehoseClient pbfirehose.FetchClient,
	connClose func() error,
	requestInfo *firehoseRequestInfo,
	err error,
) {
	return getFirehoseClientFromCmd[B, pbfirehose.FetchClient](cmd, logger, "fetch-client", endpoint, chain)
}

func getFirehoseStreamClientFromCmd[B firecore.Block](cmd *cobra.Command, logger *zap.Logger, endpoint string, chain *firecore.Chain[B]) (
	firehoseClient pbfirehose.StreamClient,
	connClose func() error,
	requestInfo *firehoseRequestInfo,
	err error,
) {
	return getFirehoseClientFromCmd[B, pbfirehose.StreamClient](cmd, logger, "stream-client", endpoint, chain)
}

func getFirehoseClientFromCmd[B firecore.Block, C any](cmd *cobra.Command, logger *zap.Logger, kind string, endpoint string, chain *firecore.Chain[B]) (
	firehoseClient C,
	connClose func() error,
	requestInfo *firehoseRequestInfo,
	err error,
) {
	requestInfo = &firehoseRequestInfo{}

	jwt := os.Getenv(sflags.MustGetString(cmd, "api-token-env-var"))
	apiKey := os.Getenv(sflags.MustGetString(cmd, "api-key-env-var"))

	plaintext := sflags.MustGetBool(cmd, "plaintext")
	insecure := sflags.MustGetBool(cmd, "insecure")

	if sflags.FlagDefined(cmd, "cursor") {
		requestInfo.Cursor = sflags.MustGetString(cmd, "cursor")
	}

	if sflags.FlagDefined(cmd, "final-blocks-only") {
		requestInfo.FinalBlocksOnly = sflags.MustGetBool(cmd, "final-blocks-only")
	}

	var rawClient any
	if kind == "stream-client" {
		rawClient, connClose, requestInfo.GRPCCallOpts, err = client.NewFirehoseClient(endpoint, jwt, apiKey, insecure, plaintext)
	} else if kind == "fetch-client" {
		rawClient, connClose, requestInfo.GRPCCallOpts, err = client.NewFirehoseFetchClient(endpoint, jwt, apiKey, insecure, plaintext)
	} else {
		panic(fmt.Errorf("unsupported Firehose client kind: %s", kind))
	}

	if err != nil {
		return firehoseClient, nil, nil, err
	}

	firehoseClient = rawClient.(C)

	compression := sflags.MustGetString(cmd, "compression")
	var compressor grpc.CallOption
	switch compression {
	case "gzip":
		compressor = grpc.UseCompressor(gzip.Name)
	case "zstd":
		compressor = grpc.UseCompressor(zstd.Name)
	case "none":
		// Valid value but nothing to do
	default:
		return firehoseClient, nil, nil, fmt.Errorf("invalid value for compression: only 'gzip', 'zstd' or 'none' are accepted")

	}

	if compressor != nil {
		requestInfo.GRPCCallOpts = append(requestInfo.GRPCCallOpts, compressor)
	}

	if chain.Tools.TransformFlags != nil {
		requestInfo.Transforms, err = chain.Tools.TransformFlags.Parse(cmd, logger)
	}

	if err != nil {
		return firehoseClient, nil, nil, fmt.Errorf("unable to parse transforms flags: %w", err)
	}

	return
}

func addFirehoseStreamClientFlagsToSet[B firecore.Block](flags *pflag.FlagSet, chain *firecore.Chain[B]) {
	addFirehoseFetchClientFlagsToSet(flags, chain)

	flags.String("cursor", "", "Use this cursor with the request to resume your stream at the following block pointed by the cursor")
}

func addFirehoseFetchClientFlagsToSet[B firecore.Block](flags *pflag.FlagSet, chain *firecore.Chain[B]) {
	flags.StringP("api-token-env-var", "a", "FIREHOSE_API_TOKEN", "Look for a JWT in this environment variable to authenticate against endpoint (alternative to api-key-env-var)")
	flags.String("api-key-env-var", "FIREHOSE_API_KEY", "Look for an API key directly in this environment variable to authenticate against endpoint (alternative to api-token-env-var)")
	flags.String("compression", "none", "The HTTP compression: use either 'none', 'gzip' or 'zstd'")
	flags.BoolP("plaintext", "p", false, "Use plaintext connection to Firehose")
	flags.BoolP("insecure", "k", false, "Use SSL connection to Firehose but skip SSL certificate validation")
	if chain.Tools.TransformFlags != nil {
		chain.Tools.TransformFlags.Register(flags)
	}
}
