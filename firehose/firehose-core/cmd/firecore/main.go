package main

import (
	pbbstream "github.com/streamingfast/bstream/pb/sf/bstream/v1"
	firecore "github.com/streamingfast/firehose-core"
	fhCMD "github.com/streamingfast/firehose-core/cmd"
)

func main() {
	firecore.UnsafeRunningFromFirecore = true
	firecore.UnsafeAllowExecutableNameToBeEmpty = true

	fhCMD.Main(&firecore.Chain[*pbbstream.Block]{
		ShortName:            "core",
		LongName:             "CORE", //only used to compose cmd title and description
		FullyQualifiedModule: "github.com/streamingfast/firehose-core",
		Version:              version,
		FirstStreamableBlock: 1,
		BlockFactory:         func() firecore.Block { return new(pbbstream.Block) },
		ConsoleReaderFactory: firecore.NewConsoleReader,
		Tools:                &firecore.ToolsConfig[*pbbstream.Block]{},
		DefaultBlockType:     "sf.fuel.type.v1.Block",
	})
}

// Version value, injected via go build `ldflags` at build time, **must** not be removed or inlined
var version = "dev"
