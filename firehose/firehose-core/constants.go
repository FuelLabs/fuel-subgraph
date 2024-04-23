package firecore

// Those are `var` and globally available so that some chains to keep backward-compatibility can
// change them. This is not advertised and should **not** be used by new chain.
var (
	MaxUint64 = ^uint64(0)
	// Common ports
	MetricsListenAddr string = ":9102"

	// Firehose chain specific port
	IndexBuilderServiceAddr        string = ":10009"
	ReaderNodeGRPCAddr             string = ":10010"
	ReaderNodeManagerAPIAddr       string = ":10011"
	MergerServingAddr              string = ":10012"
	RelayerServingAddr             string = ":10014"
	FirehoseGRPCServingAddr        string = ":10015"
	SubstreamsTier1GRPCServingAddr string = ":10016"
	SubstreamsTier2GRPCServingAddr string = ":10017"

	// Data storage default locations
	BlocksCacheDirectory string = "file://{data-dir}/storage/blocks-cache"
	MergedBlocksStoreURL string = "file://{data-dir}/storage/merged-blocks"
	OneBlockStoreURL     string = "file://{data-dir}/storage/one-blocks"
	ForkedBlocksStoreURL string = "file://{data-dir}/storage/forked-blocks"
	IndexStoreURL        string = "file://{data-dir}/storage/index"
)
