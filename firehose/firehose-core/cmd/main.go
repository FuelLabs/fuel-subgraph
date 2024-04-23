package cmd

import (
	"fmt"
	"os"
	"strings"
	"time"

	"github.com/spf13/cobra"
	"github.com/spf13/pflag"
	"github.com/spf13/viper"
	"github.com/streamingfast/cli"
	"github.com/streamingfast/cli/sflags"
	dauthgrpc "github.com/streamingfast/dauth/grpc"
	dauthnull "github.com/streamingfast/dauth/null"
	dauthsecret "github.com/streamingfast/dauth/secret"
	dauthtrust "github.com/streamingfast/dauth/trust"
	"github.com/streamingfast/dmetering"
	dmeteringgrpc "github.com/streamingfast/dmetering/grpc"
	dmeteringlogger "github.com/streamingfast/dmetering/logger"
	firecore "github.com/streamingfast/firehose-core"
	"github.com/streamingfast/firehose-core/cmd/apps"
	"github.com/streamingfast/firehose-core/cmd/tools"
	"github.com/streamingfast/firehose-core/launcher"

	"github.com/streamingfast/logging"
	"go.uber.org/zap"
)

var rootCmd = &cobra.Command{}
var rootLog *zap.Logger
var rootTracer logging.Tracer

// Main is the main entry point that configures everything and should be called from your Go
// 'main' entrypoint directly.
func Main[B firecore.Block](chain *firecore.Chain[B]) {
	dauthgrpc.Register()
	dauthnull.Register()
	dauthsecret.Register()
	dauthtrust.Register()
	dmeteringgrpc.Register()
	dmeteringlogger.Register()
	dmetering.RegisterNull()

	chain.Validate()
	chain.Init()

	binaryName := chain.BinaryName()
	rootLog, rootTracer = logging.RootLogger(binaryName, chain.RootLoggerPackageID())

	cobra.OnInitialize(func() {
		cli.ConfigureViperForCommand(rootCmd, strings.ToUpper(binaryName))

		// Compatibility to fetch `viper.GetXXX(....)` without `start-` prefix for flags on startCmd
		apps.StartCmd.LocalFlags().VisitAll(func(flag *pflag.Flag) {
			viper.BindPFlag(flag.Name, flag)
			viper.BindEnv(sflags.MustGetViperKeyFromFlag(flag), strings.ToUpper(binaryName+"_"+strings.ReplaceAll(flag.Name, "-", "_")))
		})
	})

	rootCmd.Use = binaryName
	rootCmd.Short = fmt.Sprintf("Firehose on %s", chain.LongName)
	rootCmd.Version = chain.VersionString()

	rootCmd.AddCommand(apps.StartCmd)
	rootCmd.AddCommand(tools.ToolsCmd)

	(func(flags *pflag.FlagSet) {
		flags.StringP("data-dir", "d", "./firehose-data", "Path to data storage for all components of the Firehose stack")
		flags.StringP("config-file", "c", "./firehose.yaml", "Configuration file to use. No config file loaded if set to an empty string.")

		flags.String("log-format", "text", "Format for logging to stdout. Either 'text' or 'stackdriver'")
		flags.Bool("log-to-file", true, "Also write logs to {data-dir}/firehose.log.json ")
		flags.String("log-level-switcher-listen-addr", "localhost:1065", cli.FlagDescription(`
			If non-empty, a JSON based HTTP server will listen on this address to let you switch the default logging level
			of all registered loggers to a different one on the fly. This enables switching to debug level on
			a live running production instance. Use 'curl -XPUT -d '{"level":"debug","inputs":"*"} http://localhost:1065' to
			switch the level for all loggers. Each logger (even in transitive dependencies, at least those part of the core
			StreamingFast's Firehose) are registered using two identifiers, the overarching component usually all loggers in a
			library uses the same component name like 'bstream' or 'merger', and a fully qualified ID which is usually the Go
			package fully qualified name in which the logger is defined. The 'inputs' can be either one or many component's name
			like 'bstream|merger|firehose' or a regex that is matched against the fully qualified name. If there is a match for a
			given logger, it will change its level to the one specified in 'level' field. The valid levels are 'trace', 'debug',
			'info', 'warn', 'error', 'panic'. Can be used to silence loggers by using 'panic' (well, technically it's not a full
			silence but almost), or make them more verbose and change it back later.
		`))
		flags.CountP("log-verbosity", "v", "Enables verbose output (-vvvv for max verbosity)")

		flags.String("metrics-listen-addr", ":9102", "If non-empty, the process will listen on this address to server the Prometheus metrics collected by the components.")
		flags.String("pprof-listen-addr", "localhost:6060", "If non-empty, the process will listen on this address for pprof analysis (see https://golang.org/pkg/net/http/pprof/)")
		flags.Duration("startup-delay", 0, cli.FlagDescription(`
			Delay before launching the components defined in config file or via the command line arguments. This can be used to perform
			maintenance operations on a running container or pod prior it will actually start processing. Useful for example to clear
			a persistent disks of its content before starting, cleary cached content to try to resolve bugs, etc.
		`))
	})(rootCmd.PersistentFlags())

	registerCommonFlags(chain)
	apps.RegisterReaderNodeApp(chain, rootLog)
	apps.RegisterReaderNodeStdinApp(chain, rootLog)
	apps.RegisterMergerApp(rootLog)
	apps.RegisterRelayerApp(rootLog)
	apps.RegisterFirehoseApp(chain, rootLog)
	apps.RegisterSubstreamsTier1App(chain, rootLog)
	apps.RegisterSubstreamsTier2App(chain, rootLog)

	if len(chain.BlockIndexerFactories) > 0 {
		apps.RegisterIndexBuilderApp(chain, rootLog)
	}

	startFlags := apps.StartCmd.Flags()

	if chain.RegisterExtraStartFlags != nil {
		chain.RegisterExtraStartFlags(startFlags)
	}

	if chain.ReaderNodeBootstrapperFactory != nil && startFlags.Lookup("reader-node-bootstrap-data-url") == nil {
		startFlags.String("reader-node-bootstrap-data-url", "", firecore.DefaultReaderNodeBootstrapDataURLFlagDescription())
	}

	apps.ConfigureStartCmd(chain, binaryName, rootLog)

	if err := tools.ConfigureToolsCmd(chain, rootLog, rootTracer); err != nil {
		exitWithError("registering tools command", err)
	}

	if err := launcher.RegisterFlags(rootLog, apps.StartCmd); err != nil {
		exitWithError("registering application flags", err)
	}

	var availableCmds []string
	for app := range launcher.AppRegistry {
		availableCmds = append(availableCmds, app)
	}

	apps.StartCmd.SetHelpTemplate(fmt.Sprintf(startCmdHelpTemplate, strings.Join(availableCmds, "\n  ")))
	apps.StartCmd.Example = fmt.Sprintf("%s start reader-node", binaryName)

	rootCmd.PersistentPreRunE = func(cmd *cobra.Command, _ []string) error {
		if err := setupCmd(cmd, chain.BinaryName()); err != nil {
			return err
		}

		startupDelay := viper.GetDuration("global-startup-delay")
		if startupDelay > 0 {
			rootLog.Info("sleeping before starting apps", zap.Duration("delay", startupDelay))
			time.Sleep(startupDelay)
		}

		return nil
	}

	if err := rootCmd.Execute(); err != nil {
		exitWithError("failed to run", err)
	}
}

func exitWithError(message string, err error) {
	rootLog.Error(message, zap.Error(err))
	rootLog.Sync()
	os.Exit(1)
}

func registerCommonFlags[B firecore.Block](chain *firecore.Chain[B]) {
	launcher.RegisterCommonFlags = func(_ *zap.Logger, cmd *cobra.Command) error {
		// Common stores configuration flags
		cmd.Flags().String("common-one-block-store-url", firecore.OneBlockStoreURL, "[COMMON] Store URL to read/write one-block files")
		cmd.Flags().String("common-merged-blocks-store-url", firecore.MergedBlocksStoreURL, "[COMMON] Store URL where to read/write merged blocks.")
		cmd.Flags().String("common-forked-blocks-store-url", firecore.ForkedBlocksStoreURL, "[COMMON] Store URL where to read/write forked block files that we want to keep.")
		cmd.Flags().String("common-live-blocks-addr", firecore.RelayerServingAddr, "[COMMON] gRPC endpoint to get real-time blocks.")

		cmd.Flags().String("common-index-store-url", firecore.IndexStoreURL, "[COMMON] Store URL where to read/write index files (if used on the chain).")
		cmd.Flags().IntSlice("common-index-block-sizes", []int{100000, 10000, 1000, 100}, "Index bundle sizes that that are considered valid when looking for block indexes")

		cmd.Flags().Bool("common-blocks-cache-enabled", false, cli.FlagDescription(`
			[COMMON] Use a disk cache to store the blocks data to disk and instead of keeping it in RAM. By enabling this, block's Protobuf content, in bytes,
			is kept on file system instead of RAM. This is done as soon the block is downloaded from storage. This is a tradeoff between RAM and Disk, if you
			are going to serve only a handful of concurrent requests, it's suggested to keep is disabled, if you encounter heavy RAM consumption issue, specially
			by the firehose component, it's definitely a good idea to enable it and configure it properly through the other 'common-blocks-cache-...' flags. The cache is
			split in two portions, one keeping N total bytes of blocks of the most recently used blocks and the other one keeping the N earliest blocks as
			requested by the various consumers of the cache.
		`))
		cmd.Flags().String("common-blocks-cache-dir", firecore.BlocksCacheDirectory, cli.FlagDescription(`
			[COMMON] Blocks cache directory where all the block's bytes will be cached to disk instead of being kept in RAM.
			This should be a disk that persists across restarts of the Firehose component to reduce the the strain on the disk
			when restarting and streams reconnects. The size of disk must at least big (with a 10%% buffer) in bytes as the sum of flags'
			value for  'common-blocks-cache-max-recent-entry-bytes' and 'common-blocks-cache-max-entry-by-age-bytes'.
		`))
		cmd.Flags().Int("common-blocks-cache-max-recent-entry-bytes", 21474836480, cli.FlagDescription(`
			[COMMON] Blocks cache max size in bytes of the most recently used blocks, after the limit is reached, blocks are evicted from the cache.
		`))
		cmd.Flags().Int("common-blocks-cache-max-entry-by-age-bytes", 21474836480, cli.FlagDescription(`
			[COMMON] Blocks cache max size in bytes of the earliest used blocks, after the limit is reached, blocks are evicted from the cache.
		`))

		cmd.Flags().Int("common-first-streamable-block", int(chain.FirstStreamableBlock), "[COMMON] First streamable block of the chain")

		// Authentication, metering and rate limiter plugins
		cmd.Flags().String("common-auth-plugin", "null://", "[COMMON] Auth plugin URI, see streamingfast/dauth repository")
		cmd.Flags().String("common-metering-plugin", "null://", "[COMMON] Metering plugin URI, see streamingfast/dmetering repository")

		// System Behavior
		cmd.Flags().Uint64("common-auto-mem-limit-percent", 0, "[COMMON] Automatically sets GOMEMLIMIT to a percentage of memory limit from cgroup (useful for container environments)")
		cmd.Flags().Bool("common-auto-max-procs", false, "[COMMON] Automatically sets GOMAXPROCS to max cpu available from cgroup (useful for container environments)")
		cmd.Flags().Duration("common-system-shutdown-signal-delay", 0, cli.FlagDescription(`
			[COMMON] Add a delay between receiving SIGTERM signal and shutting down apps.
			Apps will respond negatively to /healthz during this period
		`))
		return nil
	}
}

var startCmdHelpTemplate = `Usage:{{if .Runnable}}
  {{.UseLine}}{{end}} [all|command1 [command2...]]{{if gt (len .Aliases) 0}}

Aliases:
  {{.NameAndAliases}}{{end}}{{if .HasExample}}

Examples:
  {{.Example}}{{end}}

Available Commands:
  %s{{if .HasAvailableLocalFlags}}

Flags:
{{.LocalFlags.FlagUsages | trimTrailingWhitespaces}}{{end}}{{if .HasAvailableInheritedFlags}}

Global Flags:
{{.InheritedFlags.FlagUsages | trimTrailingWhitespaces}}{{end}}{{if .HasHelpSubCommands}}

Additional help topics:{{range .Commands}}{{if .IsAdditionalHelpTopicCommand}}
  {{rpad .CommandPath .CommandPathPadding}} {{.Short}}{{end}}{{end}}{{end}}{{if .HasAvailableSubCommands}}

Use "{{.CommandPath}} [command] --help" for more information about a command.{{end}}
`
