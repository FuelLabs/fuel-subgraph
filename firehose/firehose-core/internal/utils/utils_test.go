package utils

import (
	"fmt"
	"os"
	"strconv"
	"testing"

	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	"github.com/stretchr/testify/assert"
)

func TestGetEnvForceFinalityAfterBlocks(t *testing.T) {
	// Set up test case
	expected := uint64(10)
	os.Setenv("FORCE_FINALITY_AFTER_BLOCKS", strconv.FormatUint(expected, 10))
	defer os.Unsetenv("FORCE_FINALITY_AFTER_BLOCKS")

	// Call the function
	result := GetEnvForceFinalityAfterBlocks()

	// Check the result
	if result == nil {
		t.Errorf("Expected non-nil result, got nil")
	} else if *result != expected {
		t.Errorf("Expected %d, got %d", expected, *result)
	}
}
func TestTweakBlockFinality(t *testing.T) {
	// Define test cases
	testCases := []struct {
		blk                *pbbstream.Block
		maxDistanceToBlock uint64
		expectedLibNum     uint64
	}{
		{
			blk: &pbbstream.Block{
				Number: 100,
				LibNum: 80,
			},
			maxDistanceToBlock: 10,
			expectedLibNum:     90,
		},
		{
			blk: &pbbstream.Block{
				Number: 100,
				LibNum: 80,
			},
			maxDistanceToBlock: 200,
			expectedLibNum:     80,
		},
		{
			blk: &pbbstream.Block{
				Number: 100,
				LibNum: 0,
			},
			maxDistanceToBlock: 200,
			expectedLibNum:     0,
		},
		{
			blk: &pbbstream.Block{
				Number: 100,
				LibNum: 0,
			},
			maxDistanceToBlock: 10,
			expectedLibNum:     90,
		},
	}

	for i, tc := range testCases {
		t.Run(fmt.Sprintf("%d", i), func(t *testing.T) {
			TweakBlockFinality(tc.blk, tc.maxDistanceToBlock)
			assert.Equal(t, tc.expectedLibNum, tc.blk.LibNum)
		})
	}
}
