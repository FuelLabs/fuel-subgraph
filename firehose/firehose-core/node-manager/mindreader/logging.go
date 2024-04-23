package mindreader

import "os"

var traceEnabled bool

func init() {
	traceEnabled = os.Getenv("TRACE") == "true"
}
