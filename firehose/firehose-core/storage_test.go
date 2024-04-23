package firecore

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func Test_searchBlockNum(t *testing.T) {
	tests := []struct {
		name          string
		startBlockNum uint64
		lastBlockNum  *uint64
		expect        int
		expectErr     bool
	}{
		{"golden path", 1_690_600, uptr(208_853_300), 208_853_300, false},
		{"no block file found", 1_690_600, nil, 1_690_600, false},
		{"block file greater then start block", 0, uptr(100), 100, false},
		{"block file less then start block", 200, uptr(100), 200, false},
		{"golden path 2", 0, uptr(17821900), 17821900, false},
	}
	for _, tt := range tests {
		t.Run("", func(t *testing.T) {
			dstoreOpt := 0
			v, err := searchBlockNum(tt.startBlockNum, func(i uint64) (bool, error) {
				dstoreOpt++
				if tt.lastBlockNum == nil {
					return false, nil
				}
				if i > *tt.lastBlockNum {
					return false, nil
				}
				return true, nil
			})
			if tt.expectErr {
				require.Error(t, err)
			} else {
				require.NoError(t, err)
				assert.Equal(t, tt.expect, int(v))
			}
		})
	}
}

func uptr(v uint64) *uint64 {
	return &v
}
