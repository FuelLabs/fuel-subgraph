package firecore

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	"github.com/streamingfast/cli"
)

func mkdirStorePathIfLocal(storeURL string) (err error) {
	if dirs := getDirsToMake(storeURL); len(dirs) > 0 {
		err = MakeDirs(dirs)
	}

	return
}

func getDirsToMake(storeURL string) []string {
	parts := strings.Split(storeURL, "://")
	if len(parts) > 1 {
		if parts[0] != "file" {
			// Not a local store, nothing to do
			return nil
		}
		storeURL = parts[1]
	}

	// Some of the store URL are actually a file directly, let's try our best to cope for that case
	filename := filepath.Base(storeURL)
	if strings.Contains(filename, ".") {
		storeURL = filepath.Dir(storeURL)
	}

	// If we reach here, it's a local store path
	return []string{storeURL}
}

func MakeDirs(directories []string) error {
	for _, directory := range directories {
		err := os.MkdirAll(directory, 0755)
		if err != nil {
			return fmt.Errorf("failed to create directory %q: %w", directory, err)
		}
	}

	return nil
}

// MustReplaceDataDir replaces `{data-dir}` from within the `in` received argument by the
// `dataDir` argument
func MustReplaceDataDir(dataDir, in string) string {
	d, err := filepath.Abs(dataDir)
	if err != nil {
		panic(fmt.Errorf("file path abs: %w", err))
	}

	in = strings.Replace(in, "{data-dir}", d, -1)

	// Some legacy code still uses '{sf-data-dir}' (firehose-ethereum/firehose-near for example), so let's replace it
	// also to keep it compatible even though it's not advertised anymore
	in = strings.Replace(in, "{sf-data-dir}", d, -1)

	return in
}

var Example = func(in string) string {
	return string(cli.Example(in))
}

func ExamplePrefixed[B Block](chain *Chain[B], prefix, in string) string {
	return string(cli.ExamplePrefixed(chain.BinaryName()+" "+prefix, in))
}

func MustParseUint64(s string) uint64 {
	i, err := strconv.Atoi(s)
	cli.NoError(err, "Unable to parse %q as uint64", s)

	return uint64(i)
}
