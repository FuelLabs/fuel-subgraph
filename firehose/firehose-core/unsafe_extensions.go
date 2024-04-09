package firecore

import (
	"context"

	"github.com/streamingfast/firehose-core/launcher"
	"go.uber.org/zap"
)

// UnsafeRunningFromFirecore is used internally and should not be altered.
var UnsafeRunningFromFirecore = false

// UnsafeAllowedExecutableNameToBeEmpty is used internally and should not be altered.
var UnsafeAllowExecutableNameToBeEmpty = false

// UnsafeResolveReaderNodeStartBlock is a function that resolved the reader node start block num, by default it simply
// returns the value of the 'reader-node-start-block-num'. However, the function may be overwritten in certain chains
// to perform a more complex resolution logic.
var UnsafeResolveReaderNodeStartBlock = func(ctx context.Context, startBlockNum uint64, firstStreamableBlock uint64, runtime *launcher.Runtime, rootLog *zap.Logger) (uint64, error) {
	return startBlockNum, nil
}
