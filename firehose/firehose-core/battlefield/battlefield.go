package battlefield

import (
	"fmt"

	"github.com/spf13/cobra"
	"github.com/streamingfast/cli"
	. "github.com/streamingfast/cli"
)

func BattlefieldCmd(binaryName string, config *Config) cli.CommandOption {
	return Group(
		"battlefield",
		"Battlefield regression tests commands",
		Command(
			runE,
			"run",
			"Run battlefield regression test suite against oracle",
			RangeArgs(0, 1),
		),
	)
}

func runE(cmd *cobra.Command, args []string) error {
	variant := ""
	if len(args) > 0 {
		variant = args[0]
	}

	fmt.Println("Variant", variant)
	return nil
}
