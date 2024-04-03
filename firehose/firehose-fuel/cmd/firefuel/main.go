package main

import (
	"github.com/FuelLabs/firehose-fuel/codec"
	pbfuel "github.com/FuelLabs/firehose-fuel/pb/sf/fuel/type/v1"
	"github.com/spf13/pflag"
	firecore "github.com/streamingfast/firehose-core"
)

func main() {
	firecore.Main(&firecore.Chain[*pbfuel.Block]{
		ShortName:            "fuel",
		LongName:             "Fuel",
		ExecutableName:       "fuel-todo",
		FullyQualifiedModule: "github.com/FuelLabs/firehose-fuel",
		Version:              version,

		Protocol:        "FLC",
		ProtocolVersion: 1,

		FirstStreamableBlock: 1,

		BlockFactory:         func() firecore.Block { return new(pbfuel.Block) },
		ConsoleReaderFactory: codec.NewConsoleReader,

		Tools: &firecore.ToolsConfig[*pbfuel.Block]{
			BlockPrinter: printBlock,
		},

		RegisterExtraStartFlags: func(flags *pflag.FlagSet) {
			flags.String("reader-node-bootstrap-data-url", "", "URL (file or gs) to either a genesis.json file or a .tar.zst archive to decompress in the datadir. Only used when bootstrapping (no prior data)")
			flags.StringArray("substreams-rpc-endpoints", nil, "Remote endpoints to contact to satisfy Substreams 'eth_call's")
			flags.String("substreams-rpc-cache-store-url", "{data-dir}/rpc-cache", "where rpc cache will be store call responses")
			flags.Uint64("substreams-rpc-cache-chunk-size", uint64(1_000), "RPC cache chunk size in block")
		},
	})
}

// Version value, injected via go build `ldflags` at build time, **must** not be removed or inlined
var version = "dev"
