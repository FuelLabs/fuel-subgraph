package cmd

import (
	"fmt"
	_ "net/http/pprof"
	"os"
	"strings"

	"github.com/streamingfast/firehose-core/cmd/apps"

	"github.com/spf13/cobra"
	"github.com/spf13/pflag"
	"github.com/spf13/viper"
	"github.com/streamingfast/cli/sflags"
	"github.com/streamingfast/firehose-core/launcher"
)

func setupCmd(cmd *cobra.Command, binaryName string) error {
	cmd.SilenceUsage = true

	cmds := extractCmd(cmd)
	subCommand := cmds[len(cmds)-1]

	forceConfigOn := []*cobra.Command{apps.StartCmd}
	logToFileOn := []*cobra.Command{apps.StartCmd}

	if configFile := viper.GetString("global-config-file"); configFile != "" {
		exists, err := fileExists(configFile)
		if err != nil {
			return fmt.Errorf("unable to check if config file exists: %w", err)
		}

		if !exists && isMatchingCommand(cmds, forceConfigOn) {
			return fmt.Errorf("unable to find config file %q", configFile)
		}

		if exists {
			if err := launcher.LoadConfigFile(configFile); err != nil {
				return fmt.Errorf("unable to read config file %q: %w", configFile, err)
			}
		}
	}

	startFlagByName := getStartFlags()

	subconf := launcher.Config[subCommand]
	if subconf != nil {
		for k, v := range subconf.Flags {
			flag, found := startFlagByName[k]
			if !found {
				return fmt.Errorf("invalid flag %s in config file under command %s", k, subCommand)
			}

			viper.SetDefault(flag.viperKey, v)

			// For root command, we want to keep compatibility for `viper.GetXXX("global-<flag>")` to work with config loaded value
			if strings.HasPrefix(flag.viperKey, "global.") {
				viper.SetDefault(strings.Replace(flag.viperKey, "global.", "global-", 1), v)
			}

			// For 'start' command, we want to keep compatibility for `viper.GetXXX("<flag>")` to work with config loaded value
			if strings.HasPrefix(flag.viperKey, "start.") {
				viper.SetDefault(strings.TrimPrefix(flag.viperKey, "start."), v)
			}
		}
	}

	launcher.SetupLogger(rootLog, &launcher.LoggingOptions{
		WorkingDir: viper.GetString("global-data-dir"),
		// We add +1 so our default verbosity is to show all packages in INFO mode
		Verbosity:     viper.GetInt("global-log-verbosity") + 1,
		LogFormat:     viper.GetString("global-log-format"),
		LogToFile:     isMatchingCommand(cmds, logToFileOn) && viper.GetBool("global-log-to-file"),
		LogListenAddr: viper.GetString("global-log-level-switcher-listen-addr"),
		LogToStderr:   true,
	})

	launcher.SetupTracing(binaryName)
	launcher.SetupAnalyticsMetrics(rootLog, viper.GetString("global-metrics-listen-addr"), viper.GetString("global-pprof-listen-addr"))
	launcher.SetAutoMemoryLimit(viper.GetUint64("common-auto-mem-limit-percent"), rootLog)

	if viper.GetBool("common-auto-max-procs") {
		launcher.SetAutoMaxProcs(rootLog)
	}

	return nil
}

func isMatchingCommand(cmds []string, runSetupOn []*cobra.Command) bool {
	for _, c := range runSetupOn {
		baseChunks := extractCmd(c)
		if strings.Join(cmds, ".") == strings.Join(baseChunks, ".") {
			return true
		}
	}
	return false
}

func extractCmd(cmd *cobra.Command) []string {
	cmds := []string{}
	for {
		if cmd == nil {
			break
		}
		cmds = append(cmds, cmd.Use)
		cmd = cmd.Parent()
	}

	out := make([]string, len(cmds))

	for itr, v := range cmds {
		newIndex := len(cmds) - 1 - itr
		out[newIndex] = v
	}
	return out
}

func fileExists(file string) (bool, error) {
	stat, err := os.Stat(file)
	if os.IsNotExist(err) {
		return false, nil
	}

	if err != nil {
		return false, err
	}

	return !stat.IsDir(), nil
}

type flagInfo struct {
	originalName string
	viperKey     string
}

func getStartFlags() (byName map[string]*flagInfo) {
	byName = make(map[string]*flagInfo)

	rootCmd.PersistentFlags().VisitAll(func(flag *pflag.Flag) {
		byName[flag.Name] = &flagInfo{flag.Name, sflags.MustGetViperKeyFromFlag(flag)}
	})

	apps.StartCmd.LocalFlags().VisitAll(func(flag *pflag.Flag) {
		byName[flag.Name] = &flagInfo{flag.Name, sflags.MustGetViperKeyFromFlag(flag)}
	})

	return byName
}
