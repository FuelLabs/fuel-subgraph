package types

import (
	"fmt"
	"strings"

	"github.com/spf13/cobra"
	"github.com/streamingfast/bstream"
	"github.com/streamingfast/cli/sflags"
)

// GetBlockRangeFromArg returns the block range from the given argument or the range
// [HEAD, +∞] if the argument is empty.
func GetBlockRangeFromArg(in string) (out BlockRange, err error) {
	return ParseBlockRangeDefault(in, bstream.GetProtocolFirstStreamableBlock, NewOpenRange(-1))
}

// GetBlockRangeFromArgDefault returns a block range from a string argument, using the default block range
// `defaultRange` if the input is empty.
func GetBlockRangeFromArgDefault(in string, defaultRange BlockRange) (out BlockRange, err error) {
	return ParseBlockRangeDefault(in, bstream.GetProtocolFirstStreamableBlock, defaultRange)
}

// GetBlockRangeFromFlag returns the block range from the given flag name or the range
// [HEAD, +∞] if the flag is not set.
func GetBlockRangeFromFlag(cmd *cobra.Command, flagName string) (out BlockRange, err error) {
	return GetBlockRangeFromFlagDefault(cmd, flagName, NewOpenRange(-1))
}

// GetBlockRangeFromFlagDefault returns a block range from a flag, using the default block range
// `defaultRange` if the flag is not set at all.
func GetBlockRangeFromFlagDefault(cmd *cobra.Command, flagName string, defaultRange BlockRange) (out BlockRange, err error) {
	stringRange := sflags.MustGetString(cmd, flagName)

	rawRanges := strings.Split(stringRange, ",")
	if len(rawRanges) == 0 {
		return defaultRange, nil
	}

	if len(rawRanges) > 1 {
		return out, fmt.Errorf("accepting a single range for now, got %d", len(rawRanges))
	}

	out, err = ParseBlockRangeDefault(rawRanges[0], bstream.GetProtocolFirstStreamableBlock, defaultRange)
	if err != nil {
		return out, fmt.Errorf("decode range: %w", err)
	}

	return
}
