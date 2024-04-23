package firecore

import (
	"context"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"

	"github.com/spf13/cobra"
	"github.com/streamingfast/cli"
	"github.com/streamingfast/cli/sflags"
	"github.com/streamingfast/firehose-core/node-manager/operator"
	"go.uber.org/zap"
)

type ReaderNodeBootstrapperFactory func(
	ctx context.Context,
	logger *zap.Logger,
	cmd *cobra.Command,
	resolvedNodeArguments []string,
	resolver ReaderNodeArgumentResolver,
) (operator.Bootstrapper, error)

// noOpReaderNodeBootstrapperFactory is a factory that returns always `nil, nil` and is used
// as an empty override for the default bootstrapper logic.
func noOpReaderNodeBootstrapperFactory(ctx context.Context, logger *zap.Logger, cmd *cobra.Command, resolvedNodeArguments []string, resolver ReaderNodeArgumentResolver) (operator.Bootstrapper, error) {
	return nil, nil
}

func DefaultReaderNodeBootstrapDataURLFlagDescription() string {
	return cli.Dedent(`
		When specified, if the reader node is emtpy (e.g. that 'reader-node-data-dir' location doesn't exist
		or has no file within it), the 'reader-node' is going to be boostrapped from it. The exact bootstrapping
		behavior depends on the URL received.

		If the bootstrap URL is of the form 'bash:///<path/to/script>?<parameters>', the bash script at
		'<path/to/script>' will be executed. The script is going to receive in environment variables the resolved
		reader node variables in the form of 'READER_NODE_<VARIABLE_NAME>'. The fully resolved node arguments
		(from 'reader-node-arguments') are passed as args to the bash script. The query parameters accepted are:

			- arg=<value> | Pass as extra argument to the script, prepended to the list of resolved node arguments
			- env=<key>%%3d<value> | Pass as extra environment variable as <key>=<value> with key being upper-cased (multiple(s) allowed)
			- env_<key>=<value> | Pass as extra environment variable as <key>=<value> with key being upper-cased (multiple(s) allowed)
			- cwd=<path> | Change the working directory to <path> before running the script
			- interpreter=<path> | Use <path> as the interpreter to run the script
			- interpreter_arg=<arg> | Pass <interpreter_arg> as arguments to the interpreter before the script path (multiple(s) allowed)

		If the bootstrap URL ends with 'tar.zst' or 'tar.zstd', the archive is read and extracted into the
		'reader-node-data-dir' location. The archive is expected to contain the full content of the 'reader-node-data-dir'
		and is expanded as is.
	`) + "\n"
}

// DefaultReaderNodeBootstrapper is a constrtuction you can when you want the default bootstrapper logic to be applied
// but you need support new bootstrap data URL(s) format or override the default behavior for some type.
//
// The `overrideFactory` argument is a factory function that will be called first, if it returns a non-nil bootstrapper,
// it will be used and the default logic will be skipped. If it returns nil, the default logic will be applied.
func DefaultReaderNodeBootstrapper(
	overrideFactory ReaderNodeBootstrapperFactory,
) ReaderNodeBootstrapperFactory {
	return func(
		ctx context.Context,
		logger *zap.Logger,
		cmd *cobra.Command,
		resolvedNodeArguments []string,
		resolver ReaderNodeArgumentResolver,
	) (operator.Bootstrapper, error) {
		bootstrapDataURL := sflags.MustGetString(cmd, "reader-node-bootstrap-data-url")
		if bootstrapDataURL == "" {
			return nil, nil
		}

		nodeDataDir := resolver("{node-data-dir}")

		if overrideFactory == nil {
			panic("overrideFactory argument must be set")
		}

		bootstrapper, err := overrideFactory(ctx, logger, cmd, resolvedNodeArguments, resolver)
		if err != nil {
			return nil, fmt.Errorf("override factory failed: %w", err)
		}

		if bootstrapper != nil {
			return bootstrapper, nil
		}

		// Otherwise apply the default logic
		switch {
		case strings.HasSuffix(bootstrapDataURL, "tar.zst") || strings.HasSuffix(bootstrapDataURL, "tar.zstd"):
			// There could be a mistmatch here if the user override `--datadir` manually, we live it for now
			return NewTarballReaderNodeBootstrapper(bootstrapDataURL, nodeDataDir, logger), nil

		case strings.HasPrefix(bootstrapDataURL, "bash://"):
			return NewBashNodeReaderBootstrapper(cmd, bootstrapDataURL, resolver, resolvedNodeArguments, logger), nil

		default:
			return nil, fmt.Errorf("'reader-node-bootstrap-data-url' config should point to either an archive ending in '.tar.zstd' or a genesis file ending in '.json', not %s", bootstrapDataURL)
		}
	}
}

func isBootstrapped(dataDir string, logger *zap.Logger) bool {
	var foundFile bool
	err := filepath.Walk(dataDir,
		func(_ string, info os.FileInfo, err error) error {
			if err != nil {
				return err
			}
			if info.IsDir() {
				return nil
			}

			// As soon as there is a file, we assume it's bootstrapped
			foundFile = true
			return io.EOF
		})
	if err != nil && !os.IsNotExist(err) && err != io.EOF {
		logger.Warn("error while checking for bootstrapped status", zap.Error(err))
	}

	return foundFile
}
