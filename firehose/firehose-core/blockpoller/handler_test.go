package blockpoller

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFireBlockHandler_clean(t *testing.T) {
	tests := []struct {
		in     string
		expect string
	}{
		{"type.googleapis.com/sf.bstream.v2.Block", "sf.bstream.v2.Block"},
		{"sf.bstream.v2.Block", "sf.bstream.v2.Block"},
	}

	for _, test := range tests {
		t.Run(test.in, func(t *testing.T) {
			assert.Equal(t, test.expect, clean(test.in))
		})
	}

}
