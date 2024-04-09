package apps

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func Test_buildNodeArguments(t *testing.T) {
	dataDir := "/data"
	nodeDataDir := "/data/node"
	hostname := "host"

	tests := []struct {
		name          string
		args          string
		want          []string
		startBlockNum uint64
		stopBlockNum  uint64
		assertion     require.ErrorAssertionFunc
	}{
		{"no variables", "arg1 arg2", []string{"arg1", "arg2"}, 10, 20, require.NoError},
		{"variable data-dir", "{data-dir} arg2", []string{"/data", "arg2"}, 10, 20, require.NoError},
		{"variable node-data-dir", "{node-data-dir} arg2", []string{"/data/node", "arg2"}, 10, 20, require.NoError},
		{"variable hostname", "{hostname} arg2", []string{"host", "arg2"}, 10, 20, require.NoError},
		{"variable start block num", "{start-block-num} arg2", []string{"10", "arg2"}, 10, 20, require.NoError},
		{"variable stop block num", "{stop-block-num} arg2", []string{"20", "arg2"}, 10, 20, require.NoError},
		{"variable data-dir double quotes", `"{hostname} with spaces" arg2`, []string{"host with spaces", "arg2"}, 10, 20, require.NoError},
		{"variable all", `--home="{data-dir}" --data={node-data-dir} --id={hostname} --other --start={start-block-num} -stop {stop-block-num} --foo`, []string{
			"--home=/data",
			"--data=/data/node",
			"--id=host",
			"--other",
			"--start=10",
			"-stop",
			"20",
			"--foo",
		}, 10, 20, require.NoError},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			resolver := createNodeArgumentsResolver(dataDir, nodeDataDir, hostname, tt.startBlockNum, tt.stopBlockNum)
			args, err := buildNodeArguments(tt.args, resolver)
			tt.assertion(t, err)

			assert.Equal(t, tt.want, args)
		})
	}
}
