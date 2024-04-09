package types

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// func Test_BlockRange_String(t *testing.T) {
// }

func Test_readBlockRange(t *testing.T) {
	errorIs := func(errString string) require.ErrorAssertionFunc {
		return func(tt require.TestingT, err error, i ...interface{}) {
			require.EqualError(tt, err, errString, i...)
		}
	}

	headBlock := int64(HeadBlockNum)
	noRange := BlockRange{}

	openRange := NewOpenRange
	closedRange := NewClosedRange

	type args struct {
		chainFirstStreamableBlock uint64
		blockRangeArg             string
	}
	tests := []struct {
		name      string
		args      args
		want      BlockRange
		assertion require.ErrorAssertionFunc
	}{
		// Single
		{"single empty is full range", args{5, ""}, openRange(headBlock), nil},
		{"single -1 is full range", args{5, "-1"}, openRange(headBlock), nil},
		{"single : is open range from chain genesis", args{5, ":"}, openRange(5), nil},
		{"single is start block", args{5, "11"}, openRange(11), nil},
		{"single is start block and can be negative", args{5, "-2"}, openRange(-2), nil},

		{"range start, stop", args{5, "10:12"}, closedRange(10, 12), nil},
		{"range <empty>, stop", args{5, ":12"}, closedRange(5, 12), nil},
		{"range start, <empty>", args{5, "10:"}, openRange(10), nil},
		{"range start, stop+", args{5, ":+10"}, closedRange(5, 15), nil},
		{"range start, stop+", args{5, "10:+10"}, closedRange(10, 20), nil},
		{"range, equal start & end", args{0, "10:10"}, closedRange(10, 10), nil},
		{"range start, stop == -1", args{5, "10:-1"}, openRange(10), nil},

		{"error range start, stop-", args{5, "10:-2"}, noRange, errorIs("invalid range: stop block of a range cannot be negative")},
		{"error range start+, <empty>", args{5, "+10:"}, noRange, errorIs("invalid range: start block of a range cannot be positively relative (so starting with a + sign)")},
		{"error range start+, stop", args{5, "+10:20"}, noRange, errorIs("invalid range: start block of a range cannot be positively relative (so starting with a + sign)")},
		{"error single +relative is stop block, start inferred", args{5, "+10"}, noRange, errorIs("invalid range: a single block cannot be positively relative (so starting with a + sign)")},
		{"error range start+, stop+", args{5, "+10:+10"}, noRange, errorIs("invalid range: start block of a range cannot be positively relative (so starting with a + sign)")},
		{"error invalid range, over", args{0, "11:10"}, noRange, errorIs("invalid range: start block 11 is above stop block 10 (inclusive)")},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := ParseBlockRange(tt.args.blockRangeArg, tt.args.chainFirstStreamableBlock)

			if tt.assertion == nil {
				tt.assertion = require.NoError
			}

			tt.assertion(t, err)
			assert.Equal(t, tt.want, got)
		})
	}
}
