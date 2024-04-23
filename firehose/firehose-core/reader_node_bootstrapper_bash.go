package firecore

import (
	"context"
	"fmt"
	"net/url"
	"os"
	"os/exec"
	"strings"
	"time"

	"github.com/spf13/cobra"
	"github.com/streamingfast/cli/sflags"
	"go.uber.org/zap"
)

func NewBashNodeReaderBootstrapper(
	cmd *cobra.Command,
	url string,
	resolver ReaderNodeArgumentResolver,
	resolvedNodeArguments []string,
	logger *zap.Logger,
) *BashNodeBootstrapper {
	dataDir := resolver("{node-data-dir}")
	binaryPath := sflags.MustGetString(cmd, "reader-node-path")

	return &BashNodeBootstrapper{
		url:                   url,
		dataDir:               dataDir,
		binaryPath:            binaryPath,
		resolver:              resolver,
		resolvedNodeArguments: resolvedNodeArguments,
		logger:                logger,
	}
}

type BashNodeBootstrapper struct {
	url                   string
	dataDir               string
	binaryPath            string
	resolver              ReaderNodeArgumentResolver
	resolvedNodeArguments []string
	logger                *zap.Logger
}

func (b *BashNodeBootstrapper) isBootstrapped() bool {
	return isBootstrapped(b.dataDir, b.logger)
}

func (b *BashNodeBootstrapper) Bootstrap() error {
	if b.isBootstrapped() {
		return nil
	}

	b.logger.Info("bootstrapping chain data from bash script", zap.String("bootstrap_data_url", b.url))

	url, err := url.Parse(b.url)
	if err != nil {
		return fmt.Errorf("cannot parse bootstrap data URL %q: %w", b.url, err)
	}

	// Should not happen but let's play safe and check the scheme for now
	if url.Scheme != "bash" {
		return fmt.Errorf("unsupported bootstrap data URL scheme %q", url.Scheme)
	}

	parameters := url.Query()
	scriptPath := b.scriptPath(url)

	interpreter := "bash"
	if interpreterOverride := parameters.Get("interpreter"); interpreterOverride != "" {
		interpreter = interpreterOverride
	}
	workingDirectory := parameters.Get("cwd")

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Minute)
	defer cancel()

	args := []string{}
	// First arguments are those to be passed to the interpeter
	args = append(args, parameters["interpreter_arg"]...)
	// Then the argument is always the script path
	args = append(args, scriptPath)
	// Next arguments are those provided in the URL query part with the `arg` key (multiple(s) allowed)
	args = append(args, parameters["arg"]...)
	// Then we append the resolved node arguments
	args = append(args, b.resolvedNodeArguments...)

	cmd := exec.CommandContext(ctx, interpreter, args...)
	if workingDirectory != "" {
		cmd.Dir = workingDirectory
	}

	cmd.Env = os.Environ()
	cmd.Env = append(cmd.Env, b.scriptCustomEnv(url, parameters)...)
	cmd.Env = append(cmd.Env, b.nodeVariablesToEnv()...)
	cmd.Env = append(cmd.Env, fmt.Sprintf("READER_NODE_BINARY_PATH=%s", b.binaryPath))

	// Everything goes to `os.Stderr`, standard logging in nodes is to go to `os.Stderr`
	// so by default, we output everything to `os.Stderr`.
	cmd.Stdout = os.Stderr
	cmd.Stderr = os.Stderr

	if err := cmd.Run(); err != nil {
		return fmt.Errorf("bootstrap script %q failed: %w", cmd.String(), err)
	}

	return nil
}

func (b *BashNodeBootstrapper) scriptPath(bootstrapURL *url.URL) string {
	scriptPath := bootstrapURL.EscapedPath()

	// We handle relative paths differently to ensure they are properly
	// resolved to the current working directory.
	if strings.HasPrefix(scriptPath, "/.") {
		return scriptPath[1:]
	}

	return scriptPath
}

// scriptCustomEnv returns the custom environment variables to be set when running the bootstrap script
// based on the URL query parameters.
//
// We support actually two form(s): `env=<key>%3d<value>` and `env_<key>=<value>`. When ecountering
// the first form, the key is made uppercase and the value is URL-decoded and we appendd them to the
// environment. When encountering the second form, we trim the `env_` prefix, make the key uppercase
// and append the value to the environment.
func (b *BashNodeBootstrapper) scriptCustomEnv(bootstrapURL *url.URL, parameters url.Values) (out []string) {
	for k, values := range parameters {
		for _, value := range values {
			if k == "env" {
				// We assume the full element is already URL-decoded
				parts := strings.SplitN(value, "=", 2)
				if len(parts) != 2 {
					b.logger.Warn("invalid env URL query parameter", zap.String("key", k), zap.String("value", value))
					continue
				}

				out = append(out, fmt.Sprintf("%s=%s", strings.ToUpper(parts[0]), parts[1]))
			}

			if strings.HasPrefix(k, "env_") {
				out = append(out, fmt.Sprintf("%s=%s", strings.ToUpper(k[4:]), value))
			}
		}
	}

	return
}

func (b *BashNodeBootstrapper) nodeVariablesToEnv() []string {
	variablesValues := ReaderNodeVariablesValues(b.resolver)
	if len(variablesValues) == 0 {
		return nil
	}

	i := 0
	env := make([]string, len(variablesValues))
	for k, v := range variablesValues {
		env[i] = fmt.Sprintf("%s=%s", variableNameToEnvName(k), v)
		i++
	}

	return env
}

func variableNameToEnvName(variable string) string {
	name := strings.ToUpper(variable)
	name = strings.ReplaceAll(name, "-", "_")
	name = strings.TrimSpace(name)
	name = strings.TrimPrefix(name, "{")
	name = strings.TrimSuffix(name, "}")

	return "READER_NODE_" + name
}
