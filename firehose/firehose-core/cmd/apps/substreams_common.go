package apps

import (
	"sync"

	"github.com/spf13/cobra"
)

var registerSSOnce sync.Once

func registerCommonSubstreamsFlags(cmd *cobra.Command) {
	registerSSOnce.Do(func() {
		cmd.Flags().Uint64("substreams-state-bundle-size", uint64(1_000), "Interval in blocks at which to save store snapshots and output caches")
		cmd.Flags().String("substreams-state-store-url", "{sf-data-dir}/localdata", "where substreams state data are stored")
		cmd.Flags().String("substreams-state-store-default-tag", "", "If non-empty, will be appended to {substreams-state-store-url} (ex: 'v1'). Can be overriden per-request with 'X-Sf-Substreams-Cache-Tag' header")
	})
}
