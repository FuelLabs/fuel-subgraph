// Copyright 2021 dfuse Platform Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package check

import (
	"strconv"

	"github.com/spf13/cobra"
	"github.com/streamingfast/cli/sflags"
	"github.com/streamingfast/firehose-core/types"
)

func newCheckMergedBlockBatchCmd() *cobra.Command {
	var toolsCheckMergedBlocksBatchCmd = &cobra.Command{
		Use:   "merged-blocks-batch <store-url> <start> <stop>",
		Short: "Checks for any missing, disordered or duplicate blocks in merged blocks files",
		Args:  cobra.ExactArgs(3),
		RunE:  checkMergedBlocksBatchRunE,
	}
	toolsCheckMergedBlocksBatchCmd.PersistentFlags().String("output-to-store", "", "If non-empty, an empty file called <blocknum>.broken will be created for every problematic merged-blocks-file. This is a convenient way to gather the results from multiple parallel processes.")
	return toolsCheckMergedBlocksBatchCmd

}

func checkMergedBlocksBatchRunE(cmd *cobra.Command, args []string) error {
	storeURL := args[0]
	start, err := strconv.ParseUint(args[1], 10, 64)
	if err != nil {
		return err
	}
	stop, err := strconv.ParseUint(args[2], 10, 64)
	if err != nil {
		return err
	}
	fileBlockSize := uint64(100)

	blockRange := types.BlockRange{
		Start: int64(start),
		Stop:  &stop,
	}

	resultsStoreURL := sflags.MustGetString(cmd, "output-to-store")

	return CheckMergedBlocksBatch(cmd.Context(), storeURL, resultsStoreURL, fileBlockSize, blockRange)
}
