package operator

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestParseKVConfigString(t *testing.T) {
	cases := []struct {
		name        string
		in          string
		expected    map[string]string
		expectError bool
	}{
		{
			"vanilla",
			"type=pitreos store=file:///var/backups",
			map[string]string{"type": "pitreos", "store": "file:///var/backups"},
			false,
		},
		{
			"missing type",
			"store=file:///var/backups",
			nil,
			true,
		},
		{
			"empty type",
			"type= store=file:///var/backups",
			nil,
			true,
		},
		{
			"empty",
			"",
			nil,
			true,
		},
		{
			"invalid",
			"type=blah store=file:///var/backups something",
			nil,
			true,
		},
		{
			"multispace_ok",
			"type=blah    store=file:///var/backups   ",
			map[string]string{"type": "blah", "store": "file:///var/backups"},
			false,
		},
		{
			"emptystring ok",
			"type=blah    store= freq=",
			map[string]string{"type": "blah", "store": "", "freq": ""},
			false,
		},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			out, err := parseKVConfigString(tc.in)
			if tc.expectError {
				require.Error(t, err)
				return
			}

			require.NoError(t, err)
			assert.Equal(t, tc.expected, out)
		})
	}
}
