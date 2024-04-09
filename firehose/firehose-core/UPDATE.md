# Update

## v1.2.5

If you were using `github.com/streamingfast/firehose-core/firehose/client.NewFirehoseFetchClient` or `github.com/streamingfast/firehose-core/firehose/client.NewFirehoseClient`, this will be a minor breaking change, the constructor now returns `callOpts` which should be passed to all request you make, see usage in [resp, err := firehoseClient.Block(ctx, req, requestInfo.GRPCCallOpts...)](https://github.com/streamingfast/firehose-core/blob/6c20f33f74be1abe58fbded1c178223702dcd6dd/cmd/tools/firehose/single_block_client.go#L74-L75) and populating `requestInfo.GRPCCallOpts` from `NewFirehoseFetchClient` function returns can be seen [here](https://github.com/streamingfast/firehose-core/blob/develop/cmd/tools/firehose/firehose.go#L71).

## v1.0.0

No changes was required.

## v0.2.0

This includes changes that are needed to upgrade to version v0.2.0.

- The `nodemanager.GetLogLevelFunc` global override has been removed. If you had a global assignment of the form `nodemanager.GetLogLevelFunc = ...` you must removed it now. There is no replacement has the functionality this was used for has been removed. See the [version v0.2.0](./CHANGELOG.md#v020) section of the CHANGELOG for further details about the removal.

- The `firecore.Config#ReaderNodeBootstrapperFactory` has change signature from `func(cmd *cobra.Command, nodeDataDir string) (operator.Bootstrapper, error)` to `	ReaderNodeBootstrapperFactory func(ctx context.Context, logger *zap.Logger, cmd *cobra.Command, resolvedNodeArguments []string, resolver ReaderNodeArgumentResolver) (operator.Bootstrapper, error)`. The new signature provides more data than before namely you get a `context.Context` for long running operation, the `logger` instance for this app if you need to log stuff, the `cmd` if you need to extract flags, the currently fully resolved arguments via `resolvedNodeArguments` and also a `resolver` function so you can perform further replacement on your own custom flags if needed. If you were using `nodeDataDir` value before, it's easy to get it back with `nodeDataDir := resolver("{node-data-dir}")`.

  From:

  ```go
  func newReaderNodeBootstrapper(cmd *cobra.Command, nodeDataDir string) (operator.Bootstrapper, error) {
    ...
  }
  ```

  To

  ```go
  func newReaderNodeBootstrapper(ctx context.Context,
    logger *zap.Logger,
    cmd *cobra.Command,
    resolvedNodeArguments []string,
    resolver firecore.ReaderNodeArgumentResolver,
  ) (operator.Bootstrapper, error) {
    nodeDataDir := resolver("{node-data-dir}")

    ...
  }
  ```

- The `firecore.Block` has a new method that you **must** implement: `GetFirehoseBlockVersion() int32`. Implementing like:

  ```go
  func (b *Block) GetFirehoseBlockVersion() int32 {
    return N
  }
  ```

  Where `N` is the value your have currently as `ProtocolVersion` in your `Chain` config. You should **not** try to use `Chain.ProtocolVersion` directly and you must instead "hard-code" it for now. Those two values were tied together in `firecore <= v0.1.z` which was wrong so since they don't represent the same thing (one is used for the low-level file format `dbin` while the other is used in `bstream.PayloadVersion`).

- The `firecore.Block` interface method `GetFirehoseBlockLIBNum` has been removed from the core interface `firecore.Block` and has been moved instead to an optional interface `firecore.BlockLIBNumDerivable`. This was needed because not all chain convey the LIBNum within the `Block` model itself. This is not a breaking change and requires no change on your part. This was done to accomodate Ethereum and probably others.

- The `firecore.NewGenericBlockEncoder(protocolVersion int32)` has been renamed and changed signature, you should now use simply `firecore.NewBlockEncoder()`

- The way tools transform flags are registered changed to become more generic. The `firecore.ToolsConfig#TransformFlags` changed it's definition completely. If you had `firecore.Config#Tools { TransformFlags: ... }` defined, you must change a bit how you define it. You will now manually define the flags and the parser will change to parse all of the flags instead of one flag at a time.

Here the example that was applied to `firehose-near` `firecore.Chain` config:

  From:

  ```go
  TransformFlags: map[string]*firecore.TransformFlag{
    "receipt-account-filters": {
      Description: "Comma-separated accounts to use as filter/index. If it contains a colon (:), it will be interpreted as <prefix>:<suffix> (each of which can be empty, ex: 'hello:' or ':world')",
      Parser:      parseReceiptAccountFilters,
    },
  },
  ```

  To:

  ```go
  TransformFlags: &firecore.TransformFlags{
    Register: func(flags *pflag.FlagSet) {
      flags.String("receipt-account-filters", "", "Comma-separated accounts to use as filter/index. If it contains a colon (:), it will be interpreted as <prefix>:<suffix> (each of which can be empty, ex: 'hello:' or ':world')")
    },

    Parse: parseReceiptAccountFilters,
  },
  ```

  > [!NOTE]
  > Notice the rename from `Parser` to `Parse`!

  And the `parseReceiptAccountFilters` needs some update too:

  From:

  ```go
  func parseReceiptAccountFilters(in string) (*anypb.Any, error) {
    ...

    return anypb.New(filters)
  }
  ```

  To:

  ```go
  func parseReceiptAccountFilters(cmd *cobra.Command, logger *zap.Logger) ([]*anypb.Any, error) {
	  in := sflags.MustGetString(cmd, "receipt-filter-accounts")

    ...

    any, err := anypb.New(filters)
    // Deal with error

    return []*anypb.Any{any}, err
  }
  ```