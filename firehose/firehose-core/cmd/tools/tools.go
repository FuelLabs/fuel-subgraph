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

package tools

import (
	"fmt"

	"github.com/spf13/cobra"
	firecore "github.com/streamingfast/firehose-core"
	"github.com/streamingfast/firehose-core/cmd/tools/check"
	"github.com/streamingfast/firehose-core/cmd/tools/compare"
	"github.com/streamingfast/firehose-core/cmd/tools/firehose"
	"github.com/streamingfast/firehose-core/cmd/tools/fix"
	"github.com/streamingfast/firehose-core/cmd/tools/mergeblock"
	print2 "github.com/streamingfast/firehose-core/cmd/tools/print"
	"github.com/streamingfast/logging"
	"go.uber.org/zap"
)

var ToolsCmd = &cobra.Command{Use: "tools", Short: "Developer tools for operators and developers"}

func ConfigureToolsCmd[B firecore.Block](
	chain *firecore.Chain[B],
	logger *zap.Logger,
	tracer logging.Tracer,
) error {

	ToolsCmd.AddCommand(check.NewCheckCommand(chain, logger))
	ToolsCmd.AddCommand(print2.NewToolsPrintCmd(chain))

	ToolsCmd.AddCommand(compare.NewToolsCompareBlocksCmd(chain))
	ToolsCmd.AddCommand(firehose.NewToolsDownloadFromFirehoseCmd(chain, logger))
	ToolsCmd.AddCommand(firehose.NewToolsFirehoseClientCmd(chain, logger))
	ToolsCmd.AddCommand(firehose.NewToolsFirehoseSingleBlockClientCmd(chain, logger, tracer))
	ToolsCmd.AddCommand(firehose.NewToolsFirehosePrometheusExporterCmd(chain, logger, tracer))
	ToolsCmd.AddCommand(mergeblock.NewToolsUnmergeBlocksCmd(chain, logger))
	ToolsCmd.AddCommand(mergeblock.NewToolsMergeBlocksCmd(chain, logger))
	ToolsCmd.AddCommand(fix.NewToolsFixBloatedMergedBlocks(chain, logger))

	if chain.Tools.MergedBlockUpgrader != nil {
		ToolsCmd.AddCommand(mergeblock.NewToolsUpgradeMergedBlocksCmd(chain, logger))
	}

	if chain.Tools.RegisterExtraCmd != nil {
		if err := chain.Tools.RegisterExtraCmd(chain, ToolsCmd, logger, tracer); err != nil {
			return fmt.Errorf("registering extra tools command: %w", err)
		}
	}

	var walkCmd func(node *cobra.Command)
	walkCmd = func(node *cobra.Command) {
		firecore.HideGlobalFlagsOnChildCmd(node)
		for _, child := range node.Commands() {
			walkCmd(child)
		}
	}
	walkCmd(ToolsCmd)

	return nil
}
