package types

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBlockRange_String(t *testing.T) {
	tests := []struct {
		name       string
		blockRange BlockRange
		want       string
	}{
		{"open range", NewOpenRange(5000), "[#5 000, +∞]"},
		{"open range head", NewOpenRange(-1), "[HEAD, +∞]"},
		{"open range head - 2", NewOpenRange(-2), "[HEAD - 2, +∞]"},
		{"closed range", NewClosedRange(5000, 10000), "[#5 000, #10 000]"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equal(t, tt.want, tt.blockRange.String())
		})
	}
}
