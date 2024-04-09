package firecore

import (
	"context"
	"fmt"
	"runtime/debug"
	"strings"

	"github.com/streamingfast/substreams/wasm"

	"github.com/spf13/cobra"
	"github.com/spf13/pflag"
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/streamingfast/firehose-core/node-manager/mindreader"
	"github.com/streamingfast/firehose-core/node-manager/operator"
	"github.com/streamingfast/logging"
	"go.uber.org/multierr"
	"go.uber.org/zap"
	"google.golang.org/protobuf/reflect/protoreflect"
	"google.golang.org/protobuf/types/known/anypb"
)

// SanitizeBlockForCompareFunc takes a chain agnostic [block] and transforms it in-place, removing fields
// that should not be compared.
type SanitizeBlockForCompareFunc func(block *pbbstream.Block) *pbbstream.Block

// Chain is the omni config object for configuring your chain specific information. It contains various
// fields that are used everywhere to properly configure the `firehose-<chain>` binary.
//
// Each field is documented about where it's used. Throughtout the different [Chain] option,
// we will use `Acme` as the chain's name placeholder, replace it with your chain name.
type Chain[B Block] struct {
	// ShortName is the short name for your Firehose on <Chain> and is usually how
	// your chain's name is represented as a diminitutive. If your chain's name is already
	// short, we suggest to keep [ShortName] and [LongName] the same.
	//
	// As an example, Firehose on Ethereum [ShortName] is `eth` while Firehose on NEAR
	// short name is `near`.
	//
	// The [ShortName] **must** be  non-empty, lower cased and must **not** contain any spaces.
	ShortName string

	// LongName is the full name of your chain and the case sensitivy of this value is respected.
	// It is used in description of command and some logging output.
	//
	// The [LongName] **must** be non-empty.
	LongName string

	// ExecutableName is the name of the binary that is used to launch a syncing full node for this chain. For example,
	// on Ethereum, the binary by default is `geth`. This is used by the `reader-node` app to specify the
	// `reader-node-binary-name` flag.
	//
	// The [ExecutableName] **must** be non-empty.
	ExecutableName string

	// FullyQualifiedModule is the Go module of your actual `firehose-<chain>` repository and should
	// correspond to the `module` line of the `go.mod` file found at the root of your **own** `firehose-<chain>`
	// repository. The value can be seen using `head -1 go.mod | sed 's/module //'`.
	//
	// The [FullyQualifiedModule] **must** be non-empty.
	FullyQualifiedModule string

	// Version represents the actual version for your Firehose on <Chain>. It should be injected
	// via and `ldflags` through your `main` package.
	//
	// The [Version] **must** be non-empty.
	Version string

	// FirstStreamableBlock represents the block number of the first block that is streamable using Firehose,
	// for example on Ethereum it's set to `0`, the genesis block's number while on Antelope it's
	// set to 2 (genesis block is 1 there but our instrumentation on this chain instruments
	// only from block #2).
	//
	// This value is actually the default value of the `--common-first-streamable-block` flag and
	// all later usages are done using the flag's value and not this value.
	//
	// So this value is actually dynamic and can be changed at runtime using the
	// `--common-first-streamable-block`.
	//
	// The [FirstStreamableBlock] should be defined but the default 0 value is good enough
	// for most chains.
	FirstStreamableBlock uint64

	// BlockFactory is a factory function that returns a new instance of your chain's Block.
	// This new instance is usually used within `firecore` to unmarshal some bytes into your
	// chain's specific block model and return a [proto.Message] fully instantiated.
	//
	// The [BlockFactory] **must** be non-nil and must return a non-nil [proto.Message].
	BlockFactory func() Block

	// ConsoleReaderFactory is the function that should return the `ConsoleReader` that knowns
	// how to transform your your chain specific Firehose instrumentation logs into the proper
	// Block model of your chain.
	//
	// The [ConsoleReaderFactory] **must** be non-nil and must return a non-nil [mindreader.ConsolerReader] or an error.
	ConsoleReaderFactory func(lines chan string, blockEncoder BlockEncoder, logger *zap.Logger, tracer logging.Tracer) (mindreader.ConsolerReader, error)

	// BlockIndexerFactories defines the set of indexes built out of Firehose blocks to be served by Firehose
	// as custom filters.
	//
	// The [BlockIndexerFactories] is optional. If set, each key must be assigned to a non-nil [BlockIndexerFactory]. For now,
	// a single factory can be specified per chain. We use a map to allow for multiple factories in the future.
	//
	// If there is no indexer factories defined, the `index-builder` app will be disabled for this chain.
	//
	// The [BlockIndexerFactories] is optional.
	BlockIndexerFactories map[string]BlockIndexerFactory[B]

	// BlockTransformerFactories defines the set of transformer that will be enabled when the client request Firehose
	// blocks.
	//
	// The [BlockTransformerFactories] is optional. If set, each key must be assigned to a non-nil
	// [BlockTransformerFactory]. Multiple transformers can be defined.
	//
	// The [BlockTransformerFactories] is optional.
	BlockTransformerFactories map[protoreflect.FullName]BlockTransformerFactory

	// RegisterExtraStartFlags is a function that is called by the `reader-node` app to allow your chain
	// to register extra custom arguments. This function is called after the common flags are registered.
	//
	// The [RegisterExtraStartFlags] function is optional and not called if nil.
	RegisterExtraStartFlags func(flags *pflag.FlagSet)

	// ReaderNodeBootstrapperFactory enables the `reader-node` app to have a custom bootstrapper for your chain.
	// By default, no specialized bootstrapper is defined.
	//
	// If this is set, the `reader-node` app will use the one bootstrapper returned by this function. The function
	// will receive the `start` command where flags are defined as well as the node's absolute data directory as an
	// argument.
	ReaderNodeBootstrapperFactory func(
		ctx context.Context,
		logger *zap.Logger,
		cmd *cobra.Command,
		resolvedNodeArguments []string,
		resolver ReaderNodeArgumentResolver,
	) (operator.Bootstrapper, error)

	// Tools aggregate together all configuration options required for the various `fire<chain> tools`
	// to work properly for example to print block using chain specific information.
	//
	// The [Tools] element is optional and if not provided, sane defaults will be used.
	Tools *ToolsConfig[B]

	// BlockEncoder is the cached block encoder object that should be used for this chain. Populate
	// when Init() is called will be `nil` prior to that.
	//
	// When you need to encode your chain specific block like `pbeth.Block` into a `bstream.Block` you
	// should use this encoder:
	//
	//     bstreamBlock, err := chain.BlockEncoder.Encode(block)
	//
	BlockEncoder BlockEncoder

	RegisterSubstreamsExtensions func() (wasm.WASMExtensioner, error)
}

type ToolsConfig[B Block] struct {
	// SanitizeBlockForCompare is a function that takes a chain agnostic [block] and transforms it in-place, removing fields
	// that should not be compared.
	//
	// The [SanitizeBlockForCompare] is optional, if nil, no-op sanitizer be used.
	SanitizeBlockForCompare SanitizeBlockForCompareFunc

	// RegisterExtraCmd enables you to register extra commands to the `fire<chain> tools` group.
	// The callback function is called with the `toolsCmd` command that is the root command of the `fire<chain> tools`
	// as well as the chain, the root logger and root tracer for tools.
	//
	// You are responsible of calling `toolsCmd.AddCommand` to register your extra commands.
	//
	// The [RegisterExtraCmd] function is optional and not called if nil.
	RegisterExtraCmd func(chain *Chain[B], toolsCmd *cobra.Command, zlog *zap.Logger, tracer logging.Tracer) error

	// TransformFlags specify chain specific transforms flags (and parsing of those flag's value). The flags defined
	// in there are added to all Firehose-client like tools commannd (`tools firehose-client`, `tools firehose-prometheus-exporter`, etc.)
	// automatically.
	//
	// Refer to the TransformFlags for further details on how respect the contract of this field.
	//
	// The [TransformFlags] is optional.
	TransformFlags *TransformFlags

	// MergedBlockUpgrader when define enables for your chain to upgrade between different versions of "merged-blocks".
	// It happens from time to time that a data bug is found in the way merged blocks and it's possible to fix it by
	// applying a transformation to the block. This is what this function is for.
	//
	// When defined, a new tools `fire<chain> tools upgrade-merged-blocks` is added. This command will enable operators
	// to upgrade from one version to another of the merged blocks.
	//
	// The [MergedBlockUpgrader] is optional and not specifying it disables command `fire<chain> tools upgrade-merged-blocks`.
	MergedBlockUpgrader func(block *pbbstream.Block) (*pbbstream.Block, error)
}

// GetSanitizeBlockForCompare returns the [SanitizeBlockForCompare] value if defined, otherwise a no-op sanitizer.
func (t *ToolsConfig[B]) GetSanitizeBlockForCompare() SanitizeBlockForCompareFunc {
	if t == nil || t.SanitizeBlockForCompare == nil {
		return func(block *pbbstream.Block) *pbbstream.Block { return block }
	}

	return t.SanitizeBlockForCompare
}

type TransformFlags struct {
	// Register is a function that will be called when we need to register the flags for the transforms.
	// You received the command's flag set and you are responsible of registering the flags.
	Register func(flags *pflag.FlagSet)

	// Parse is a function that will be called when we need to extract the transforms out of the flags.
	// You received the command and the logger and you are responsible of parsing the flags and returning
	// the transforms.
	//
	// Flags can be obtain with `sflags.MustGetString(cmd, "<flag-name>")` and you will obtain the value.
	Parse func(cmd *cobra.Command, logger *zap.Logger) ([]*anypb.Any, error)
}

// Validate normalizes some aspect of the [Chain] values (spaces trimming essentially) and validates the chain
// by accumulating error an panic if all the error found along the way.
func (c *Chain[B]) Validate() {
	c.ShortName = strings.ToLower(strings.TrimSpace(c.ShortName))
	c.LongName = strings.TrimSpace(c.LongName)
	c.ExecutableName = strings.TrimSpace(c.ExecutableName)

	var err error

	if c.ShortName == "" {
		err = multierr.Append(err, fmt.Errorf("field 'ShortName' must be non-empty"))
	}

	if strings.Contains(c.ShortName, " ") {
		err = multierr.Append(err, fmt.Errorf("field 'ShortName' must not contain any space(s)"))
	}

	if c.LongName == "" {
		err = multierr.Append(err, fmt.Errorf("field 'LongName' must be non-empty"))
	}

	if c.ExecutableName == "" && !UnsafeAllowExecutableNameToBeEmpty {
		err = multierr.Append(err, fmt.Errorf("field 'ExecutableName' must be non-empty"))
	}

	if c.FullyQualifiedModule == "" {
		err = multierr.Append(err, fmt.Errorf("field 'FullyQualifiedModule' must be non-empty"))
	}

	if c.Version == "" {
		err = multierr.Append(err, fmt.Errorf("field 'Version' must be non-empty"))
	}

	if c.BlockFactory == nil {
		err = multierr.Append(err, fmt.Errorf("field 'BlockFactory' must be non-nil"))
	} else if c.BlockFactory() == nil {
		err = multierr.Append(err, fmt.Errorf("field 'BlockFactory' must not produce nil blocks"))
	}

	if c.ConsoleReaderFactory == nil {
		err = multierr.Append(err, fmt.Errorf("field 'ConsoleReaderFactory' must be non-nil"))
	}

	if len(c.BlockIndexerFactories) > 1 {
		err = multierr.Append(err, fmt.Errorf("field 'BlockIndexerFactories' must have at most one element"))
	}

	for key, indexerFactory := range c.BlockIndexerFactories {
		if indexerFactory == nil {
			err = multierr.Append(err, fmt.Errorf("entry %q for field 'BlockIndexerFactories' must be non-nil", key))
		}
	}

	for key, transformerFactory := range c.BlockTransformerFactories {
		if transformerFactory == nil {
			err = multierr.Append(err, fmt.Errorf("entry %q for field 'BlockTransformerFactories' must be non-nil", key))
		}
	}

	errors := multierr.Errors(err)
	if len(errors) > 0 {
		errorLines := make([]string, len(errors))
		for i, err := range errors {
			errorLines[i] = fmt.Sprintf("- %s", err)
		}

		panic(fmt.Sprintf("firecore.Chain is invalid:\n%s", strings.Join(errorLines, "\n")))
	}
}

// Init is called when the chain is first loaded to initialize the `bstream`
// library with the chain specific configuration.
//
// This must called only once per chain per process.
//
// **Caveats** Two chain in the same Go binary will not work today as `bstream` uses global
// variables to store configuration which presents multiple chain to exist in the same process.
func (c *Chain[B]) Init() {
	c.BlockEncoder = NewBlockEncoder()

	if c.ReaderNodeBootstrapperFactory == nil {
		c.ReaderNodeBootstrapperFactory = DefaultReaderNodeBootstrapper(noOpReaderNodeBootstrapperFactory)
	}
}

// BinaryName represents the binary name for your Firehose on <Chain> is the [ShortName]
// lowered appended to 'fire' prefix to before for example `fireacme`.
func (c *Chain[B]) BinaryName() string {
	return "fire" + strings.ToLower(c.ShortName)
}

// RootLoggerPackageID is the `packageID` value when instantiating the root logger on the chain
// that is used by CLI command and other
func (c *Chain[B]) RootLoggerPackageID() string {
	return c.LoggerPackageID(fmt.Sprintf("cmd/%s/cli", c.BinaryName()))
}

// LoggerPackageID computes a logger `packageID` value for a specific sub-package.
func (c *Chain[B]) LoggerPackageID(subPackage string) string {
	return fmt.Sprintf("%s/%s", c.FullyQualifiedModule, subPackage)
}

// VersionString computes the version string that will be display when calling `firexxx --version`
// and extract build information from Git via Golang `debug.ReadBuildInfo`.
func (c *Chain[B]) VersionString() string {
	info, ok := debug.ReadBuildInfo()
	if !ok {
		panic("we should have been able to retrieve info from 'runtime/debug#ReadBuildInfo'")
	}

	commit := findSetting("vcs.revision", info.Settings)
	date := findSetting("vcs.time", info.Settings)

	var labels []string
	if len(commit) >= 7 {
		labels = append(labels, fmt.Sprintf("Commit %s", commit[0:7]))
	}

	if date != "" {
		labels = append(labels, fmt.Sprintf("Built %s", date))
	}

	if len(labels) == 0 {
		return c.Version
	}

	return fmt.Sprintf("%s (%s)", c.Version, strings.Join(labels, ", "))
}

func findSetting(key string, settings []debug.BuildSetting) (value string) {
	for _, setting := range settings {
		if setting.Key == key {
			return setting.Value
		}
	}

	return ""
}
