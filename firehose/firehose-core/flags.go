package firecore

import "github.com/spf13/cobra"

// globalFlagsHiddenOnChildCmd represents the list of global flags that should be hidden on child commands
var globalFlagsHiddenOnChildCmd = []string{
	"log-level-switcher-listen-addr",
	"metrics-listen-addr",
	"pprof-listen-addr",
	"startup-delay",
}

func HideGlobalFlagsOnChildCmd(cmd *cobra.Command) {
	actual := cmd.HelpFunc()
	cmd.SetHelpFunc(func(command *cobra.Command, strings []string) {
		for _, flag := range globalFlagsHiddenOnChildCmd {
			command.Flags().MarkHidden(flag)
		}

		actual(command, strings)
	})
}
