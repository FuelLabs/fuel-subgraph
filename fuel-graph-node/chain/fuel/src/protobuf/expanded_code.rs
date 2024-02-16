#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod adapter {
    use crate::{data_source::DataSource, Chain};
    use graph::{blockchain as bc, prelude::*};
    pub struct TriggerFilter {
        pub(crate) block_filter: FuelBlockFilter,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TriggerFilter {
        #[inline]
        fn clone(&self) -> TriggerFilter {
            TriggerFilter {
                block_filter: ::core::clone::Clone::clone(&self.block_filter),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TriggerFilter {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "TriggerFilter",
                "block_filter",
                &&self.block_filter,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for TriggerFilter {
        #[inline]
        fn default() -> TriggerFilter {
            TriggerFilter {
                block_filter: ::core::default::Default::default(),
            }
        }
    }
    impl bc::TriggerFilter<Chain> for TriggerFilter {
        fn extend_with_template(
            &mut self,
            _data_source: impl Iterator<
                Item = <Chain as bc::Blockchain>::DataSourceTemplate,
            >,
        ) {}
        fn extend<'a>(
            &mut self,
            data_sources: impl Iterator<Item = &'a DataSource> + Clone,
        ) {
            let TriggerFilter { block_filter } = self;
            block_filter
                .extend(FuelBlockFilter::from_data_sources(data_sources.clone()));
        }
        fn node_capabilities(&self) -> bc::EmptyNodeCapabilities<Chain> {
            bc::EmptyNodeCapabilities::default()
        }
        fn to_firehose_filter(self) -> Vec<prost_types::Any> {
            ::core::panicking::panic_fmt(
                ::core::fmt::Arguments::new_v1(
                    &["Should never be called since not used by FirehoseBlockStream"],
                    &[],
                ),
            )
        }
    }
    pub(crate) struct FuelBlockFilter {
        pub trigger_every_block: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FuelBlockFilter {
        #[inline]
        fn clone(&self) -> FuelBlockFilter {
            FuelBlockFilter {
                trigger_every_block: ::core::clone::Clone::clone(
                    &self.trigger_every_block,
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FuelBlockFilter {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "FuelBlockFilter",
                "trigger_every_block",
                &&self.trigger_every_block,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for FuelBlockFilter {
        #[inline]
        fn default() -> FuelBlockFilter {
            FuelBlockFilter {
                trigger_every_block: ::core::default::Default::default(),
            }
        }
    }
    impl FuelBlockFilter {
        pub fn from_data_sources<'a>(
            iter: impl IntoIterator<Item = &'a DataSource>,
        ) -> Self {
            Self {
                trigger_every_block: iter
                    .into_iter()
                    .any(|data_source| !data_source.mapping.block_handlers.is_empty()),
            }
        }
        pub fn extend(&mut self, other: FuelBlockFilter) {
            self
                .trigger_every_block = self.trigger_every_block
                || other.trigger_every_block;
        }
    }
}
mod capabilities {
    use std::{cmp::PartialOrd, fmt, str::FromStr};
    use anyhow::Error;
    use graph::impl_slog_value;
    pub struct NodeCapabilities {}
    #[automatically_derived]
    impl ::core::clone::Clone for NodeCapabilities {
        #[inline]
        fn clone(&self) -> NodeCapabilities {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for NodeCapabilities {}
    #[automatically_derived]
    impl ::core::fmt::Debug for NodeCapabilities {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "NodeCapabilities")
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for NodeCapabilities {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for NodeCapabilities {
        #[inline]
        fn eq(&self, other: &NodeCapabilities) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for NodeCapabilities {}
    #[automatically_derived]
    impl ::core::cmp::Eq for NodeCapabilities {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for NodeCapabilities {
        #[inline]
        fn partial_cmp(
            &self,
            other: &NodeCapabilities,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        }
    }
    impl FromStr for NodeCapabilities {
        type Err = Error;
        fn from_str(_s: &str) -> Result<Self, Self::Err> {
            Ok(NodeCapabilities {})
        }
    }
    impl fmt::Display for NodeCapabilities {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("{{CHAIN_NAME}}")
        }
    }
    impl ::graph::slog::Value for NodeCapabilities {
        fn serialize(
            &self,
            record: &::graph::slog::Record,
            key: ::graph::slog::Key,
            serializer: &mut dyn ::graph::slog::Serializer,
        ) -> ::graph::slog::Result {
            {
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&self)],
                    ),
                );
                res
            }
                .serialize(record, key, serializer)
        }
    }
}
pub mod chain {
    use graph::{
        anyhow::Result,
        blockchain::{
            block_stream::{
                BlockStreamEvent, BlockWithTriggers, FirehoseError,
                FirehoseMapper as FirehoseMapperTrait,
                TriggersAdapter as TriggersAdapterTrait,
            },
            firehose_block_stream::FirehoseBlockStream, BasicBlockchainBuilder, Block,
            BlockHash, BlockIngestor, BlockPtr, Blockchain, BlockchainBuilder,
            BlockchainKind, EmptyNodeCapabilities, IngestorError, NoopRuntimeAdapter,
            RuntimeAdapter as RuntimeAdapterTrait,
        },
        components::store::DeploymentLocator, env::EnvVars,
        firehose::{self as firehose, ForkStep},
        prelude::{
            async_trait, o, BlockNumber, ChainStore, Error, Logger, LoggerFactory,
            MetricsRegistry,
        },
        schema::InputSchema, substreams::Clock,
    };
    use prost::Message;
    use std::sync::Arc;
    use crate::{
        adapter::TriggerFilter, codec,
        data_source::{
            DataSource, DataSourceTemplate, UnresolvedDataSource,
            UnresolvedDataSourceTemplate,
        },
    };
    use graph::{
        blockchain::{
            block_stream::{
                BlockStream, BlockStreamBuilder, BlockStreamMapper, FirehoseCursor,
            },
            client::ChainClient, firehose_block_ingestor::FirehoseBlockIngestor,
        },
        cheap_clone::CheapClone, components::store::DeploymentCursorTracker,
        data::subgraph::UnifiedMappingApiVersion, firehose::FirehoseEndpoint,
    };
    use crate::trigger::FuelTrigger;
    pub struct Chain {
        logger_factory: LoggerFactory,
        name: String,
        client: Arc<ChainClient<Self>>,
        chain_store: Arc<dyn ChainStore>,
        metrics_registry: Arc<MetricsRegistry>,
        block_stream_builder: Arc<dyn BlockStreamBuilder<Self>>,
    }
    pub struct FuelStreamBuilder;
    pub struct FirehoseMapper {
        adapter: Arc<dyn TriggersAdapterTrait<Chain>>,
        filter: Arc<TriggerFilter>,
    }
    impl BlockStreamMapper<Chain> for FirehoseMapper {
        fn decode_block(
            &self,
            output: Option<&[u8]>,
        ) -> Result<Option<codec::Block>, Error> {
            let block = match output {
                Some(block) => codec::Block::decode(block)?,
                None => {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            ::core::fmt::Arguments::new_v1(
                                &["fuel mapper is expected to always have a block"],
                                &[],
                            ),
                        );
                        error
                    });
                }
            };
            Ok(Some(block))
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn block_with_triggers<'life0, 'life1, 'async_trait>(
            &'life0 self,
            logger: &'life1 Logger,
            block: codec::Block,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<BlockWithTriggers<Chain>, Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<BlockWithTriggers<Chain>, Error>,
                > {
                    return __ret;
                }
                let __self = self;
                let logger = logger;
                let block = block;
                let __ret: Result<BlockWithTriggers<Chain>, Error> = {
                    __self
                        .adapter
                        .triggers_in_block(logger, block, __self.filter.as_ref())
                        .await
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn handle_substreams_block<'life0, 'life1, 'async_trait>(
            &'life0 self,
            _logger: &'life1 Logger,
            _clock: Clock,
            _cursor: FirehoseCursor,
            _message: Vec<u8>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<BlockStreamEvent<Chain>, Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<BlockStreamEvent<Chain>, Error>,
                > {
                    return __ret;
                }
                let __self = self;
                let _logger = _logger;
                let _clock = _clock;
                let _cursor = _cursor;
                let _message = _message;
                let __ret: Result<BlockStreamEvent<Chain>, Error> = {
                    ::core::panicking::panic("not yet implemented")
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
    impl FirehoseMapperTrait<Chain> for FirehoseMapper {
        fn trigger_filter(&self) -> &TriggerFilter {
            self.filter.as_ref()
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn to_block_stream_event<'life0, 'life1, 'life2, 'async_trait>(
            &'life0 self,
            logger: &'life1 Logger,
            response: &'life2 firehose::Response,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<BlockStreamEvent<Chain>, FirehoseError>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            'life2: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<BlockStreamEvent<Chain>, FirehoseError>,
                > {
                    return __ret;
                }
                let __self = self;
                let logger = logger;
                let response = response;
                let __ret: Result<BlockStreamEvent<Chain>, FirehoseError> = {
                    let step = ForkStep::from_i32(response.step)
                        .unwrap_or_else(|| {
                            ::core::panicking::panic_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[
                                        "unknown step i32 value ",
                                        ", maybe you forgot update & re-regenerate the protobuf definitions?",
                                    ],
                                    &[::core::fmt::ArgumentV1::new_display(&response.step)],
                                ),
                            )
                        });
                    let any_block = response
                        .block
                        .as_ref()
                        .expect("block payload information should always be present");
                    let block = __self
                        .decode_block(Some(any_block.value.as_ref()))?
                        .unwrap();
                    use ForkStep::*;
                    match step {
                        StepNew => {
                            Ok(
                                BlockStreamEvent::ProcessBlock(
                                    __self.block_with_triggers(logger, block).await?,
                                    FirehoseCursor::from(response.cursor.clone()),
                                ),
                            )
                        }
                        StepUndo => {
                            let parent_ptr = block
                                .parent_ptr()
                                .expect("Genesis block should never be reverted");
                            Ok(
                                BlockStreamEvent::Revert(
                                    parent_ptr,
                                    FirehoseCursor::from(response.cursor.clone()),
                                ),
                            )
                        }
                        StepFinal => {
                            ::core::panicking::panic_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[
                                        "irreversible step is not handled and should not be requested in the Firehose request",
                                    ],
                                    &[],
                                ),
                            )
                        }
                        StepUnset => {
                            ::core::panicking::panic_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[
                                        "unknown step should not happen in the Firehose response",
                                    ],
                                    &[],
                                ),
                            )
                        }
                    }
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn block_ptr_for_number<'life0, 'life1, 'life2, 'async_trait>(
            &'life0 self,
            logger: &'life1 Logger,
            endpoint: &'life2 Arc<FirehoseEndpoint>,
            number: BlockNumber,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<BlockPtr, Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            'life2: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<BlockPtr, Error>,
                > {
                    return __ret;
                }
                let __self = self;
                let logger = logger;
                let endpoint = endpoint;
                let number = number;
                let __ret: Result<BlockPtr, Error> = {
                    endpoint
                        .block_ptr_for_number::<codec::Block>(logger, number)
                        .await
                        .map_err(Into::into)
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn final_block_ptr_for<'life0, 'life1, 'life2, 'life3, 'async_trait>(
            &'life0 self,
            logger: &'life1 Logger,
            endpoint: &'life2 Arc<FirehoseEndpoint>,
            block: &'life3 codec::Block,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<BlockPtr, Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            'life2: 'async_trait,
            'life3: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<BlockPtr, Error>,
                > {
                    return __ret;
                }
                let __self = self;
                let logger = logger;
                let endpoint = endpoint;
                let block = block;
                let __ret: Result<BlockPtr, Error> = {
                    let final_block_number = match block.number() {
                        x if x >= 200 => x - 200,
                        _ => 0,
                    };
                    __self
                        .block_ptr_for_number(logger, endpoint, final_block_number)
                        .await
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
    pub struct TriggersAdapter {}
    impl BlockStreamBuilder<Chain> for FuelStreamBuilder {
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn build_firehose<'life0, 'life1, 'async_trait>(
            &'life0 self,
            chain: &'life1 Chain,
            deployment: DeploymentLocator,
            block_cursor: FirehoseCursor,
            start_blocks: Vec<BlockNumber>,
            subgraph_current_block: Option<BlockPtr>,
            filter: Arc<TriggerFilter>,
            unified_api_version: UnifiedMappingApiVersion,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Box<dyn BlockStream<Chain>>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Box<dyn BlockStream<Chain>>>,
                > {
                    return __ret;
                }
                let __self = self;
                let chain = chain;
                let deployment = deployment;
                let block_cursor = block_cursor;
                let start_blocks = start_blocks;
                let subgraph_current_block = subgraph_current_block;
                let filter = filter;
                let unified_api_version = unified_api_version;
                let __ret: Result<Box<dyn BlockStream<Chain>>> = {
                    let adapter = chain
                        .triggers_adapter(
                            &deployment,
                            &EmptyNodeCapabilities::default(),
                            unified_api_version,
                        )
                        .unwrap_or_else(|_| ::core::panicking::panic_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["no adapter for network "],
                                &[::core::fmt::ArgumentV1::new_display(&chain.name)],
                            ),
                        ));
                    let logger = chain
                        .logger_factory
                        .subgraph_logger(&deployment)
                        .new(
                            ::slog::OwnedKV((
                                ::slog::SingleKV::from((
                                    "component",
                                    "FirehoseBlockStream",
                                )),
                                (),
                            )),
                        );
                    let firehose_mapper = Arc::new(FirehoseMapper { adapter, filter });
                    Ok(
                        Box::new(
                            FirehoseBlockStream::new(
                                deployment.hash,
                                chain.chain_client(),
                                subgraph_current_block,
                                block_cursor,
                                firehose_mapper,
                                start_blocks,
                                logger,
                                chain.metrics_registry.clone(),
                            ),
                        ),
                    )
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn build_substreams<'life0, 'life1, 'async_trait>(
            &'life0 self,
            _chain: &'life1 Chain,
            _schema: InputSchema,
            _deployment: DeploymentLocator,
            _block_cursor: FirehoseCursor,
            _subgraph_current_block: Option<BlockPtr>,
            _filter: Arc<<Chain as Blockchain>::TriggerFilter>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Box<dyn BlockStream<Chain>>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Box<dyn BlockStream<Chain>>>,
                > {
                    return __ret;
                }
                let __self = self;
                let _chain = _chain;
                let _schema = _schema;
                let _deployment = _deployment;
                let _block_cursor = _block_cursor;
                let _subgraph_current_block = _subgraph_current_block;
                let _filter = _filter;
                let __ret: Result<Box<dyn BlockStream<Chain>>> = {
                    ::core::panicking::panic("not implemented")
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn build_polling<'life0, 'life1, 'async_trait>(
            &'life0 self,
            _chain: &'life1 Chain,
            _deployment: DeploymentLocator,
            _start_blocks: Vec<BlockNumber>,
            _subgraph_current_block: Option<BlockPtr>,
            _filter: Arc<TriggerFilter>,
            _unified_api_version: UnifiedMappingApiVersion,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Box<dyn BlockStream<Chain>>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Box<dyn BlockStream<Chain>>>,
                > {
                    return __ret;
                }
                let __self = self;
                let _chain = _chain;
                let _deployment = _deployment;
                let _start_blocks = _start_blocks;
                let _subgraph_current_block = _subgraph_current_block;
                let _filter = _filter;
                let _unified_api_version = _unified_api_version;
                let __ret: Result<Box<dyn BlockStream<Chain>>> = {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["FuelNet does not support polling block stream"],
                            &[],
                        ),
                    )
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
    impl std::fmt::Debug for Chain {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(&["chain: fuel"], &[]))
        }
    }
    impl BlockchainBuilder<Chain> for BasicBlockchainBuilder {
        fn build(self, _config: &Arc<EnvVars>) -> Chain {
            Chain {
                logger_factory: self.logger_factory,
                name: self.name,
                chain_store: self.chain_store,
                client: Arc::new(ChainClient::new_firehose(self.firehose_endpoints)),
                metrics_registry: self.metrics_registry,
                block_stream_builder: Arc::new(FuelStreamBuilder {}),
            }
        }
    }
    impl Blockchain for Chain {
        const KIND: BlockchainKind = BlockchainKind::Fuel;
        type Client = ();
        type Block = codec::Block;
        type DataSource = DataSource;
        type UnresolvedDataSource = UnresolvedDataSource;
        type DataSourceTemplate = DataSourceTemplate;
        type UnresolvedDataSourceTemplate = UnresolvedDataSourceTemplate;
        type TriggerData = crate::trigger::FuelTrigger;
        type MappingTrigger = crate::trigger::FuelTrigger;
        type TriggerFilter = crate::adapter::TriggerFilter;
        type NodeCapabilities = EmptyNodeCapabilities<Self>;
        fn triggers_adapter(
            &self,
            _loc: &DeploymentLocator,
            _capabilities: &Self::NodeCapabilities,
            _unified_api_version: UnifiedMappingApiVersion,
        ) -> Result<Arc<dyn TriggersAdapterTrait<Self>>, Error> {
            let adapter = TriggersAdapter {};
            Ok(Arc::new(adapter))
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn new_block_stream<'life0, 'async_trait>(
            &'life0 self,
            deployment: DeploymentLocator,
            store: impl DeploymentCursorTracker,
            start_blocks: Vec<BlockNumber>,
            filter: Arc<Self::TriggerFilter>,
            unified_api_version: UnifiedMappingApiVersion,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Box<dyn BlockStream<Self>>, Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Box<dyn BlockStream<Self>>, Error>,
                > {
                    return __ret;
                }
                let __self = self;
                let deployment = deployment;
                let store = store;
                let start_blocks = start_blocks;
                let filter = filter;
                let unified_api_version = unified_api_version;
                let __ret: Result<Box<dyn BlockStream<Self>>, Error> = {
                    __self
                        .block_stream_builder
                        .build_firehose(
                            __self,
                            deployment,
                            store.firehose_cursor(),
                            start_blocks,
                            store.block_ptr(),
                            filter,
                            unified_api_version,
                        )
                        .await
                };
                #[allow(unreachable_code)] __ret
            })
        }
        fn chain_store(&self) -> Arc<dyn ChainStore> {
            self.chain_store.clone()
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn block_pointer_from_number<'life0, 'life1, 'async_trait>(
            &'life0 self,
            logger: &'life1 Logger,
            number: BlockNumber,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<BlockPtr, IngestorError>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<BlockPtr, IngestorError>,
                > {
                    return __ret;
                }
                let __self = self;
                let logger = logger;
                let number = number;
                let __ret: Result<BlockPtr, IngestorError> = {
                    __self
                        .client
                        .firehose_endpoint()?
                        .block_ptr_for_number::<codec::Block>(logger, number)
                        .await
                        .map_err(Into::into)
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn refetch_firehose_block<'life0, 'life1, 'async_trait>(
            &'life0 self,
            _logger: &'life1 Logger,
            _cursor: FirehoseCursor,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<codec::Block, Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<codec::Block, Error>,
                > {
                    return __ret;
                }
                let __self = self;
                let _logger = _logger;
                let _cursor = _cursor;
                let __ret: Result<codec::Block, Error> = {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["not implemented: "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &::core::fmt::Arguments::new_v1(
                                        &[
                                            "This chain does not support Dynamic Data Sources. is_refetch_block_required always returns false, this shouldn\'t be called.",
                                        ],
                                        &[],
                                    ),
                                ),
                            ],
                        ),
                    )
                };
                #[allow(unreachable_code)] __ret
            })
        }
        fn is_refetch_block_required(&self) -> bool {
            false
        }
        fn runtime_adapter(&self) -> Arc<dyn RuntimeAdapterTrait<Self>> {
            Arc::new(NoopRuntimeAdapter::default())
        }
        fn chain_client(&self) -> Arc<ChainClient<Self>> {
            self.client.clone()
        }
        fn block_ingestor(&self) -> anyhow::Result<Box<dyn BlockIngestor>> {
            let ingestor = FirehoseBlockIngestor::<
                crate::Block,
                Self,
            >::new(
                self.chain_store.cheap_clone(),
                self.chain_client(),
                self.logger_factory.component_logger("FuelFirehoseBlockIngestor", None),
                self.name.clone(),
            );
            Ok(Box::new(ingestor))
        }
    }
    impl TriggersAdapterTrait<Chain> for TriggersAdapter {
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn ancestor_block<'life0, 'async_trait>(
            &'life0 self,
            _ptr: BlockPtr,
            _offset: BlockNumber,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Option<codec::Block>, Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Option<codec::Block>, Error>,
                > {
                    return __ret;
                }
                let __self = self;
                let _ptr = _ptr;
                let _offset = _offset;
                let __ret: Result<Option<codec::Block>, Error> = {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "Should never be called since FirehoseBlockStream cannot resolve it",
                            ],
                            &[],
                        ),
                    )
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn scan_triggers<'life0, 'life1, 'async_trait>(
            &'life0 self,
            _from: BlockNumber,
            _to: BlockNumber,
            _filter: &'life1 crate::adapter::TriggerFilter,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Vec<BlockWithTriggers<Chain>>, Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Vec<BlockWithTriggers<Chain>>, Error>,
                > {
                    return __ret;
                }
                let __self = self;
                let _from = _from;
                let _to = _to;
                let _filter = _filter;
                let __ret: Result<Vec<BlockWithTriggers<Chain>>, Error> = {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "Should never be called since not used by FirehoseBlockStream",
                            ],
                            &[],
                        ),
                    )
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(unused)]
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn triggers_in_block<'life0, 'life1, 'life2, 'async_trait>(
            &'life0 self,
            logger: &'life1 Logger,
            block: codec::Block,
            filter: &'life2 TriggerFilter,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<BlockWithTriggers<Chain>, Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            'life2: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<BlockWithTriggers<Chain>, Error>,
                > {
                    return __ret;
                }
                let __self = self;
                let logger = logger;
                let block = block;
                let filter = filter;
                let __ret: Result<BlockWithTriggers<Chain>, Error> = {
                    let shared_block = Arc::new(block.clone());
                    let TriggerFilter { block_filter } = filter;
                    let mut trigger_data: Vec<_> = ::alloc::vec::Vec::new();
                    if block_filter.trigger_every_block {
                        trigger_data
                            .push(FuelTrigger::Block(shared_block.cheap_clone()));
                    }
                    Ok(BlockWithTriggers::new(block, trigger_data, logger))
                };
                #[allow(unreachable_code)] __ret
            })
        }
        /// Return `true` if the block with the given hash and number is on the
        /// main chain, i.e., the chain going back from the current chain head.
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn is_on_main_chain<'life0, 'async_trait>(
            &'life0 self,
            _ptr: BlockPtr,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<bool, Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<bool, Error>,
                > {
                    return __ret;
                }
                let __self = self;
                let _ptr = _ptr;
                let __ret: Result<bool, Error> = {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "Should never be called since not used by FirehoseBlockStream",
                            ],
                            &[],
                        ),
                    )
                };
                #[allow(unreachable_code)] __ret
            })
        }
        /// Get pointer to parent of `block`. This is called when reverting `block`.
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn parent_ptr<'life0, 'life1, 'async_trait>(
            &'life0 self,
            block: &'life1 BlockPtr,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Option<BlockPtr>, Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Option<BlockPtr>, Error>,
                > {
                    return __ret;
                }
                let __self = self;
                let block = block;
                let __ret: Result<Option<BlockPtr>, Error> = {
                    Ok(
                        Some(BlockPtr {
                            hash: BlockHash::from(::alloc::vec::from_elem(0xff, 32)),
                            number: block.number.saturating_sub(1),
                        }),
                    )
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
}
pub mod codec {
    pub use crate::protobuf::pbcodec::*;
    use graph::{
        blockchain::{Block as BlockchainBlock, BlockPtr},
        prelude::BlockNumber,
    };
    use std::convert::TryFrom;
    impl Block {
        pub fn parent_ptr(&self) -> Option<BlockPtr> {
            match self.height {
                0 => None,
                _ => {
                    Some(BlockPtr {
                        hash: self.prev_id.clone().into(),
                        number: self.number().saturating_sub(1),
                    })
                }
            }
        }
    }
    impl<'a> From<&'a Block> for BlockPtr {
        fn from(b: &'a Block) -> BlockPtr {
            BlockPtr::new(b.id.clone().into(), b.number())
        }
    }
    impl BlockchainBlock for Block {
        fn ptr(&self) -> BlockPtr {
            BlockPtr::try_from(self).unwrap()
        }
        fn parent_ptr(&self) -> Option<BlockPtr> {
            self.parent_ptr()
        }
        fn number(&self) -> i32 {
            BlockNumber::try_from(self.height).unwrap()
        }
    }
}
mod data_source {
    use anyhow::{anyhow, Context, Error};
    use graph::{
        blockchain::{self, Block, Blockchain, TriggerWithHandler},
        components::store::StoredDynamicDataSource, data::subgraph::DataSourceContext,
        prelude::{
            async_trait, BlockNumber, CheapClone, DataSourceTemplateInfo, Deserialize,
            Link, LinkResolver, Logger,
        },
        semver,
    };
    use std::{collections::HashSet, sync::Arc};
    use crate::chain::Chain;
    pub const FUEL_KIND: &str = "fuelnet";
    const BLOCK_HANDLER_KIND: &str = "block";
    /// Runtime representation of a data source.
    pub struct DataSource {
        pub kind: String,
        pub network: Option<String>,
        pub name: String,
        pub(crate) source: Source,
        pub mapping: Mapping,
        pub context: Arc<Option<DataSourceContext>>,
        pub creation_block: Option<BlockNumber>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DataSource {
        #[inline]
        fn clone(&self) -> DataSource {
            DataSource {
                kind: ::core::clone::Clone::clone(&self.kind),
                network: ::core::clone::Clone::clone(&self.network),
                name: ::core::clone::Clone::clone(&self.name),
                source: ::core::clone::Clone::clone(&self.source),
                mapping: ::core::clone::Clone::clone(&self.mapping),
                context: ::core::clone::Clone::clone(&self.context),
                creation_block: ::core::clone::Clone::clone(&self.creation_block),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DataSource {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "kind",
                "network",
                "name",
                "source",
                "mapping",
                "context",
                "creation_block",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &&self.kind,
                &&self.network,
                &&self.name,
                &&self.source,
                &&self.mapping,
                &&self.context,
                &&self.creation_block,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "DataSource",
                names,
                values,
            )
        }
    }
    impl DataSource {
        fn from_manifest(
            kind: String,
            network: Option<String>,
            name: String,
            source: Source,
            mapping: Mapping,
            context: Option<DataSourceContext>,
        ) -> Result<Self, Error> {
            let creation_block = None;
            Ok(DataSource {
                kind,
                network,
                name,
                source,
                mapping,
                context: Arc::new(context),
                creation_block,
            })
        }
        fn handler_for_block(&self) -> Option<&MappingBlockHandler> {
            self.mapping.block_handlers.first()
        }
    }
    #[serde(rename_all = "camelCase")]
    pub struct UnresolvedMapping {
        pub api_version: String,
        pub language: String,
        pub entities: Vec<String>,
        #[serde(default)]
        pub block_handlers: Vec<MappingBlockHandler>,
        pub file: Link,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UnresolvedMapping {
        #[inline]
        fn clone(&self) -> UnresolvedMapping {
            UnresolvedMapping {
                api_version: ::core::clone::Clone::clone(&self.api_version),
                language: ::core::clone::Clone::clone(&self.language),
                entities: ::core::clone::Clone::clone(&self.entities),
                block_handlers: ::core::clone::Clone::clone(&self.block_handlers),
                file: ::core::clone::Clone::clone(&self.file),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UnresolvedMapping {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "UnresolvedMapping",
                "api_version",
                &&self.api_version,
                "language",
                &&self.language,
                "entities",
                &&self.entities,
                "block_handlers",
                &&self.block_handlers,
                "file",
                &&self.file,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for UnresolvedMapping {
        #[inline]
        fn default() -> UnresolvedMapping {
            UnresolvedMapping {
                api_version: ::core::default::Default::default(),
                language: ::core::default::Default::default(),
                entities: ::core::default::Default::default(),
                block_handlers: ::core::default::Default::default(),
                file: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for UnresolvedMapping {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.api_version, state);
            ::core::hash::Hash::hash(&self.language, state);
            ::core::hash::Hash::hash(&self.entities, state);
            ::core::hash::Hash::hash(&self.block_handlers, state);
            ::core::hash::Hash::hash(&self.file, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for UnresolvedMapping {}
    #[automatically_derived]
    impl ::core::cmp::Eq for UnresolvedMapping {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<Vec<String>>;
            let _: ::core::cmp::AssertParamIsEq<Vec<MappingBlockHandler>>;
            let _: ::core::cmp::AssertParamIsEq<Link>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for UnresolvedMapping {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for UnresolvedMapping {
        #[inline]
        fn eq(&self, other: &UnresolvedMapping) -> bool {
            self.api_version == other.api_version && self.language == other.language
                && self.entities == other.entities
                && self.block_handlers == other.block_handlers && self.file == other.file
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UnresolvedMapping {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "apiVersion" => _serde::__private::Ok(__Field::__field0),
                            "language" => _serde::__private::Ok(__Field::__field1),
                            "entities" => _serde::__private::Ok(__Field::__field2),
                            "blockHandlers" => _serde::__private::Ok(__Field::__field3),
                            "file" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"apiVersion" => _serde::__private::Ok(__Field::__field0),
                            b"language" => _serde::__private::Ok(__Field::__field1),
                            b"entities" => _serde::__private::Ok(__Field::__field2),
                            b"blockHandlers" => _serde::__private::Ok(__Field::__field3),
                            b"file" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<UnresolvedMapping>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UnresolvedMapping;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct UnresolvedMapping",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UnresolvedMapping with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UnresolvedMapping with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Vec<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct UnresolvedMapping with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Vec<MappingBlockHandler>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            Link,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct UnresolvedMapping with 5 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(UnresolvedMapping {
                            api_version: __field0,
                            language: __field1,
                            entities: __field2,
                            block_handlers: __field3,
                            file: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<
                            Vec<MappingBlockHandler>,
                        > = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<Link> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "apiVersion",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "language",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "entities",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "blockHandlers",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<MappingBlockHandler>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("file"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Link>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("apiVersion")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("language")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("entities")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("file")?
                            }
                        };
                        _serde::__private::Ok(UnresolvedMapping {
                            api_version: __field0,
                            language: __field1,
                            entities: __field2,
                            block_handlers: __field3,
                            file: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "apiVersion",
                    "language",
                    "entities",
                    "blockHandlers",
                    "file",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UnresolvedMapping",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<UnresolvedMapping>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl UnresolvedMapping {
        pub async fn resolve(
            self,
            resolver: &Arc<dyn LinkResolver>,
            logger: &Logger,
        ) -> Result<Mapping, Error> {
            let UnresolvedMapping {
                api_version,
                language,
                entities,
                block_handlers,
                file: link,
            } = self;
            let api_version = semver::Version::parse(&api_version)?;
            let module_bytes = resolver
                .cat(logger, &link)
                .await
                .with_context(|| {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["failed to resolve mapping "],
                            &[::core::fmt::ArgumentV1::new_display(&link.link)],
                        ),
                    );
                    res
                })?;
            Ok(Mapping {
                api_version,
                language,
                entities,
                block_handlers,
                runtime: Arc::new(module_bytes),
                link,
            })
        }
    }
    pub struct UnresolvedDataSource {
        pub kind: String,
        pub network: Option<String>,
        pub name: String,
        pub(crate) source: Source,
        pub mapping: UnresolvedMapping,
        pub context: Option<DataSourceContext>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UnresolvedDataSource {
        #[inline]
        fn clone(&self) -> UnresolvedDataSource {
            UnresolvedDataSource {
                kind: ::core::clone::Clone::clone(&self.kind),
                network: ::core::clone::Clone::clone(&self.network),
                name: ::core::clone::Clone::clone(&self.name),
                source: ::core::clone::Clone::clone(&self.source),
                mapping: ::core::clone::Clone::clone(&self.mapping),
                context: ::core::clone::Clone::clone(&self.context),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UnresolvedDataSource {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "kind",
                "network",
                "name",
                "source",
                "mapping",
                "context",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &&self.kind,
                &&self.network,
                &&self.name,
                &&self.source,
                &&self.mapping,
                &&self.context,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "UnresolvedDataSource",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for UnresolvedDataSource {}
    #[automatically_derived]
    impl ::core::cmp::Eq for UnresolvedDataSource {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<Option<String>>;
            let _: ::core::cmp::AssertParamIsEq<Source>;
            let _: ::core::cmp::AssertParamIsEq<UnresolvedMapping>;
            let _: ::core::cmp::AssertParamIsEq<Option<DataSourceContext>>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for UnresolvedDataSource {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for UnresolvedDataSource {
        #[inline]
        fn eq(&self, other: &UnresolvedDataSource) -> bool {
            self.kind == other.kind && self.network == other.network
                && self.name == other.name && self.source == other.source
                && self.mapping == other.mapping && self.context == other.context
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UnresolvedDataSource {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "kind" => _serde::__private::Ok(__Field::__field0),
                            "network" => _serde::__private::Ok(__Field::__field1),
                            "name" => _serde::__private::Ok(__Field::__field2),
                            "source" => _serde::__private::Ok(__Field::__field3),
                            "mapping" => _serde::__private::Ok(__Field::__field4),
                            "context" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"kind" => _serde::__private::Ok(__Field::__field0),
                            b"network" => _serde::__private::Ok(__Field::__field1),
                            b"name" => _serde::__private::Ok(__Field::__field2),
                            b"source" => _serde::__private::Ok(__Field::__field3),
                            b"mapping" => _serde::__private::Ok(__Field::__field4),
                            b"context" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<UnresolvedDataSource>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UnresolvedDataSource;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct UnresolvedDataSource",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UnresolvedDataSource with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UnresolvedDataSource with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct UnresolvedDataSource with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Source,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct UnresolvedDataSource with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            UnresolvedMapping,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct UnresolvedDataSource with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            Option<DataSourceContext>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct UnresolvedDataSource with 6 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(UnresolvedDataSource {
                            kind: __field0,
                            network: __field1,
                            name: __field2,
                            source: __field3,
                            mapping: __field4,
                            context: __field5,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Source> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<UnresolvedMapping> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<
                            Option<DataSourceContext>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("kind"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "network",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("source"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Source>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "mapping",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            UnresolvedMapping,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "context",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<DataSourceContext>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("kind")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("network")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("name")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("source")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("mapping")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("context")?
                            }
                        };
                        _serde::__private::Ok(UnresolvedDataSource {
                            kind: __field0,
                            network: __field1,
                            name: __field2,
                            source: __field3,
                            mapping: __field4,
                            context: __field5,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "kind",
                    "network",
                    "name",
                    "source",
                    "mapping",
                    "context",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UnresolvedDataSource",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<UnresolvedDataSource>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl blockchain::UnresolvedDataSource<Chain> for UnresolvedDataSource {
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn resolve<'life0, 'life1, 'async_trait>(
            self,
            resolver: &'life0 Arc<dyn LinkResolver>,
            logger: &'life1 Logger,
            _manifest_idx: u32,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<DataSource, Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<DataSource, Error>,
                > {
                    return __ret;
                }
                let __self = self;
                let resolver = resolver;
                let logger = logger;
                let _manifest_idx = _manifest_idx;
                let __ret: Result<DataSource, Error> = {
                    let UnresolvedDataSource {
                        kind,
                        network,
                        name,
                        source,
                        mapping,
                        context,
                    } = __self;
                    let mapping = mapping
                        .resolve(resolver, logger)
                        .await
                        .with_context(|| {
                            {
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "failed to resolve data source ",
                                            " with source_account ",
                                            " and source_start_block ",
                                        ],
                                        &[
                                            ::core::fmt::ArgumentV1::new_display(&name),
                                            ::core::fmt::ArgumentV1::new_debug(&source.owner),
                                            ::core::fmt::ArgumentV1::new_display(&source.start_block),
                                        ],
                                    ),
                                );
                                res
                            }
                        })?;
                    DataSource::from_manifest(
                        kind,
                        network,
                        name,
                        source,
                        mapping,
                        context,
                    )
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
    pub struct BaseDataSourceTemplate<M> {
        pub kind: String,
        pub network: Option<String>,
        pub name: String,
        pub mapping: M,
    }
    #[automatically_derived]
    impl<M: ::core::clone::Clone> ::core::clone::Clone for BaseDataSourceTemplate<M> {
        #[inline]
        fn clone(&self) -> BaseDataSourceTemplate<M> {
            BaseDataSourceTemplate {
                kind: ::core::clone::Clone::clone(&self.kind),
                network: ::core::clone::Clone::clone(&self.network),
                name: ::core::clone::Clone::clone(&self.name),
                mapping: ::core::clone::Clone::clone(&self.mapping),
            }
        }
    }
    #[automatically_derived]
    impl<M: ::core::fmt::Debug> ::core::fmt::Debug for BaseDataSourceTemplate<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "BaseDataSourceTemplate",
                "kind",
                &&self.kind,
                "network",
                &&self.network,
                "name",
                &&self.name,
                "mapping",
                &&self.mapping,
            )
        }
    }
    #[automatically_derived]
    impl<M: ::core::default::Default> ::core::default::Default
    for BaseDataSourceTemplate<M> {
        #[inline]
        fn default() -> BaseDataSourceTemplate<M> {
            BaseDataSourceTemplate {
                kind: ::core::default::Default::default(),
                network: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                mapping: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl<M: ::core::hash::Hash> ::core::hash::Hash for BaseDataSourceTemplate<M> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.kind, state);
            ::core::hash::Hash::hash(&self.network, state);
            ::core::hash::Hash::hash(&self.name, state);
            ::core::hash::Hash::hash(&self.mapping, state)
        }
    }
    #[automatically_derived]
    impl<M> ::core::marker::StructuralEq for BaseDataSourceTemplate<M> {}
    #[automatically_derived]
    impl<M: ::core::cmp::Eq> ::core::cmp::Eq for BaseDataSourceTemplate<M> {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<Option<String>>;
            let _: ::core::cmp::AssertParamIsEq<M>;
        }
    }
    #[automatically_derived]
    impl<M> ::core::marker::StructuralPartialEq for BaseDataSourceTemplate<M> {}
    #[automatically_derived]
    impl<M: ::core::cmp::PartialEq> ::core::cmp::PartialEq
    for BaseDataSourceTemplate<M> {
        #[inline]
        fn eq(&self, other: &BaseDataSourceTemplate<M>) -> bool {
            self.kind == other.kind && self.network == other.network
                && self.name == other.name && self.mapping == other.mapping
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, M> _serde::Deserialize<'de> for BaseDataSourceTemplate<M>
        where
            M: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "kind" => _serde::__private::Ok(__Field::__field0),
                            "network" => _serde::__private::Ok(__Field::__field1),
                            "name" => _serde::__private::Ok(__Field::__field2),
                            "mapping" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"kind" => _serde::__private::Ok(__Field::__field0),
                            b"network" => _serde::__private::Ok(__Field::__field1),
                            b"name" => _serde::__private::Ok(__Field::__field2),
                            b"mapping" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de, M>
                where
                    M: _serde::Deserialize<'de>,
                {
                    marker: _serde::__private::PhantomData<BaseDataSourceTemplate<M>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, M> _serde::de::Visitor<'de> for __Visitor<'de, M>
                where
                    M: _serde::Deserialize<'de>,
                {
                    type Value = BaseDataSourceTemplate<M>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct BaseDataSourceTemplate",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct BaseDataSourceTemplate with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct BaseDataSourceTemplate with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct BaseDataSourceTemplate with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            M,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct BaseDataSourceTemplate with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(BaseDataSourceTemplate {
                            kind: __field0,
                            network: __field1,
                            name: __field2,
                            mapping: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<M> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("kind"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "network",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "mapping",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<M>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("kind")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("network")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("name")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("mapping")?
                            }
                        };
                        _serde::__private::Ok(BaseDataSourceTemplate {
                            kind: __field0,
                            network: __field1,
                            name: __field2,
                            mapping: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "kind",
                    "network",
                    "name",
                    "mapping",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "BaseDataSourceTemplate",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<
                            BaseDataSourceTemplate<M>,
                        >,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub type UnresolvedDataSourceTemplate = BaseDataSourceTemplate<UnresolvedMapping>;
    pub type DataSourceTemplate = BaseDataSourceTemplate<Mapping>;
    impl blockchain::UnresolvedDataSourceTemplate<Chain>
    for UnresolvedDataSourceTemplate {
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn resolve<'life0, 'life1, 'async_trait>(
            self,
            resolver: &'life0 Arc<dyn LinkResolver>,
            logger: &'life1 Logger,
            _manifest_idx: u32,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<DataSourceTemplate, Error>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<DataSourceTemplate, Error>,
                > {
                    return __ret;
                }
                let __self = self;
                let resolver = resolver;
                let logger = logger;
                let _manifest_idx = _manifest_idx;
                let __ret: Result<DataSourceTemplate, Error> = {
                    let UnresolvedDataSourceTemplate { kind, network, name, mapping } = __self;
                    let mapping = mapping
                        .resolve(resolver, logger)
                        .await
                        .with_context(|| {
                            {
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["failed to resolve data source template "],
                                        &[::core::fmt::ArgumentV1::new_display(&name)],
                                    ),
                                );
                                res
                            }
                        })?;
                    Ok(DataSourceTemplate {
                        kind,
                        network,
                        name,
                        mapping,
                    })
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
    impl blockchain::DataSourceTemplate<Chain> for DataSourceTemplate {
        fn api_version(&self) -> semver::Version {
            self.mapping.api_version.clone()
        }
        fn runtime(&self) -> Option<Arc<Vec<u8>>> {
            Some(self.mapping.runtime.cheap_clone())
        }
        fn name(&self) -> &str {
            &self.name
        }
        fn manifest_idx(&self) -> u32 {
            ::core::panicking::panic_fmt(
                ::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &[
                        ::core::fmt::ArgumentV1::new_display(
                            &::core::fmt::Arguments::new_v1(
                                &["fuel does not support dynamic data sources"],
                                &[],
                            ),
                        ),
                    ],
                ),
            )
        }
        fn kind(&self) -> &str {
            &self.kind
        }
    }
    #[serde(rename_all = "camelCase")]
    pub(crate) struct Source {
        pub(crate) owner: Option<String>,
        #[serde(default)]
        pub(crate) start_block: BlockNumber,
        pub(crate) end_block: Option<BlockNumber>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Source {
        #[inline]
        fn clone(&self) -> Source {
            Source {
                owner: ::core::clone::Clone::clone(&self.owner),
                start_block: ::core::clone::Clone::clone(&self.start_block),
                end_block: ::core::clone::Clone::clone(&self.end_block),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Source {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Source",
                "owner",
                &&self.owner,
                "start_block",
                &&self.start_block,
                "end_block",
                &&self.end_block,
            )
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Source {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.owner, state);
            ::core::hash::Hash::hash(&self.start_block, state);
            ::core::hash::Hash::hash(&self.end_block, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Source {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Source {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Option<String>>;
            let _: ::core::cmp::AssertParamIsEq<BlockNumber>;
            let _: ::core::cmp::AssertParamIsEq<Option<BlockNumber>>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Source {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Source {
        #[inline]
        fn eq(&self, other: &Source) -> bool {
            self.owner == other.owner && self.start_block == other.start_block
                && self.end_block == other.end_block
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Source {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "owner" => _serde::__private::Ok(__Field::__field0),
                            "startBlock" => _serde::__private::Ok(__Field::__field1),
                            "endBlock" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"owner" => _serde::__private::Ok(__Field::__field0),
                            b"startBlock" => _serde::__private::Ok(__Field::__field1),
                            b"endBlock" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Source>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Source;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Source",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Source with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            BlockNumber,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Option<BlockNumber>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Source with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Source {
                            owner: __field0,
                            start_block: __field1,
                            end_block: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<BlockNumber> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<
                            Option<BlockNumber>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("owner"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "startBlock",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            BlockNumber,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "endBlock",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<BlockNumber>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("owner")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("endBlock")?
                            }
                        };
                        _serde::__private::Ok(Source {
                            owner: __field0,
                            start_block: __field1,
                            end_block: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "owner",
                    "startBlock",
                    "endBlock",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Source",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Source>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct Mapping {
        pub api_version: semver::Version,
        pub language: String,
        pub entities: Vec<String>,
        pub block_handlers: Vec<MappingBlockHandler>,
        pub runtime: Arc<Vec<u8>>,
        pub link: Link,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Mapping {
        #[inline]
        fn clone(&self) -> Mapping {
            Mapping {
                api_version: ::core::clone::Clone::clone(&self.api_version),
                language: ::core::clone::Clone::clone(&self.language),
                entities: ::core::clone::Clone::clone(&self.entities),
                block_handlers: ::core::clone::Clone::clone(&self.block_handlers),
                runtime: ::core::clone::Clone::clone(&self.runtime),
                link: ::core::clone::Clone::clone(&self.link),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Mapping {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "api_version",
                "language",
                "entities",
                "block_handlers",
                "runtime",
                "link",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &&self.api_version,
                &&self.language,
                &&self.entities,
                &&self.block_handlers,
                &&self.runtime,
                &&self.link,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "Mapping",
                names,
                values,
            )
        }
    }
    pub struct MappingBlockHandler {
        pub handler: String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MappingBlockHandler {
        #[inline]
        fn clone(&self) -> MappingBlockHandler {
            MappingBlockHandler {
                handler: ::core::clone::Clone::clone(&self.handler),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MappingBlockHandler {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "MappingBlockHandler",
                "handler",
                &&self.handler,
            )
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for MappingBlockHandler {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.handler, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for MappingBlockHandler {}
    #[automatically_derived]
    impl ::core::cmp::Eq for MappingBlockHandler {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MappingBlockHandler {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MappingBlockHandler {
        #[inline]
        fn eq(&self, other: &MappingBlockHandler) -> bool {
            self.handler == other.handler
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MappingBlockHandler {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "handler" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"handler" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MappingBlockHandler>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MappingBlockHandler;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct MappingBlockHandler",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct MappingBlockHandler with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(MappingBlockHandler {
                            handler: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "handler",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("handler")?
                            }
                        };
                        _serde::__private::Ok(MappingBlockHandler {
                            handler: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["handler"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MappingBlockHandler",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MappingBlockHandler>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct TransactionHandler {
        pub handler: String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TransactionHandler {
        #[inline]
        fn clone(&self) -> TransactionHandler {
            TransactionHandler {
                handler: ::core::clone::Clone::clone(&self.handler),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TransactionHandler {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "TransactionHandler",
                "handler",
                &&self.handler,
            )
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for TransactionHandler {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.handler, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for TransactionHandler {}
    #[automatically_derived]
    impl ::core::cmp::Eq for TransactionHandler {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TransactionHandler {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TransactionHandler {
        #[inline]
        fn eq(&self, other: &TransactionHandler) -> bool {
            self.handler == other.handler
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TransactionHandler {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "handler" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"handler" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TransactionHandler>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TransactionHandler;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct TransactionHandler",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct TransactionHandler with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(TransactionHandler {
                            handler: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "handler",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("handler")?
                            }
                        };
                        _serde::__private::Ok(TransactionHandler {
                            handler: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["handler"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TransactionHandler",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TransactionHandler>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl blockchain::DataSource<Chain> for DataSource {
        fn from_template_info(
            _info: DataSourceTemplateInfo<Chain>,
        ) -> Result<Self, Error> {
            Err(
                ::anyhow::__private::must_use({
                    let error = ::anyhow::__private::format_err(
                        ::core::fmt::Arguments::new_v1(
                            &["Fuel subgraphs do not support templates"],
                            &[],
                        ),
                    );
                    error
                }),
            )
        }
        fn from_stored_dynamic_data_source(
            _template: &DataSourceTemplate,
            _stored: StoredDynamicDataSource,
        ) -> Result<Self, Error> {
            ::core::panicking::panic("not yet implemented")
        }
        fn address(&self) -> Option<&[u8]> {
            self.source.owner.as_ref().map(String::as_bytes)
        }
        fn start_block(&self) -> BlockNumber {
            self.source.start_block
        }
        fn end_block(&self) -> Option<BlockNumber> {
            self.source.end_block
        }
        fn name(&self) -> &str {
            &self.name
        }
        fn kind(&self) -> &str {
            &self.kind
        }
        fn network(&self) -> Option<&str> {
            self.network.as_deref()
        }
        fn context(&self) -> Arc<Option<DataSourceContext>> {
            self.context.cheap_clone()
        }
        fn creation_block(&self) -> Option<BlockNumber> {
            self.creation_block
        }
        fn api_version(&self) -> semver::Version {
            self.mapping.api_version.clone()
        }
        fn runtime(&self) -> Option<Arc<Vec<u8>>> {
            Some(self.mapping.runtime.cheap_clone())
        }
        fn handler_kinds(&self) -> HashSet<&str> {
            let mut kinds = HashSet::new();
            if self.handler_for_block().is_some() {
                kinds.insert(BLOCK_HANDLER_KIND);
            }
            kinds
        }
        fn match_and_decode(
            &self,
            trigger: &<Chain as Blockchain>::TriggerData,
            block: &Arc<<Chain as Blockchain>::Block>,
            _logger: &Logger,
        ) -> Result<Option<TriggerWithHandler<Chain>>, Error> {
            if self.source.start_block > block.number() {
                return Ok(None);
            }
            let handler = match self.handler_for_block() {
                Some(handler) => &handler.handler,
                None => return Ok(None),
            };
            Ok(
                Some(
                    TriggerWithHandler::<
                        Chain,
                    >::new(trigger.cheap_clone(), handler.clone(), block.ptr()),
                ),
            )
        }
        fn is_duplicate_of(&self, other: &Self) -> bool {
            let DataSource {
                kind,
                network,
                name,
                source,
                mapping,
                context,
                creation_block: _,
            } = self;
            kind == &other.kind && network == &other.network && name == &other.name
                && source == &other.source
                && mapping.block_handlers == other.mapping.block_handlers
                && context == &other.context
        }
        fn as_stored_dynamic_data_source(&self) -> StoredDynamicDataSource {
            ::core::panicking::panic("not yet implemented")
        }
        fn validate(&self) -> Vec<Error> {
            let mut errors = Vec::new();
            if self.kind != FUEL_KIND {
                errors
                    .push(
                        ::anyhow::Error::msg({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &[
                                        "data source has invalid `kind`, expected ",
                                        " but found ",
                                    ],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&FUEL_KIND),
                                        ::core::fmt::ArgumentV1::new_display(&self.kind),
                                    ],
                                ),
                            );
                            res
                        }),
                    )
            }
            if self.mapping.block_handlers.len() > 1 {
                errors
                    .push(
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                ::core::fmt::Arguments::new_v1(
                                    &["data source has duplicated block handlers"],
                                    &[],
                                ),
                            );
                            error
                        }),
                    );
            }
            errors
        }
    }
}
mod protobuf {
    #[rustfmt::skip]
    #[path = "sf.fuel.r#type.v1.rs"]
    pub mod pbcodec {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct Block {
            #[prost(bytes = "vec", tag = "1")]
            pub id: ::prost::alloc::vec::Vec<u8>,
            #[prost(uint32, tag = "2")]
            pub height: u32,
            #[prost(uint64, tag = "3")]
            pub da_height: u64,
            #[prost(uint64, tag = "4")]
            pub msg_receipt_count: u64,
            #[prost(bytes = "vec", tag = "5")]
            pub tx_root: ::prost::alloc::vec::Vec<u8>,
            #[prost(bytes = "vec", tag = "6")]
            pub msg_receipt_root: ::prost::alloc::vec::Vec<u8>,
            #[prost(bytes = "vec", tag = "7")]
            pub prev_id: ::prost::alloc::vec::Vec<u8>,
            #[prost(bytes = "vec", tag = "8")]
            pub prev_root: ::prost::alloc::vec::Vec<u8>,
            #[prost(fixed64, tag = "9")]
            pub timestamp: u64,
            #[prost(bytes = "vec", tag = "10")]
            pub application_hash: ::prost::alloc::vec::Vec<u8>,
            #[prost(message, repeated, tag = "11")]
            pub transactions: ::prost::alloc::vec::Vec<Transaction>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Block {
            #[inline]
            fn clone(&self) -> Block {
                Block {
                    id: ::core::clone::Clone::clone(&self.id),
                    height: ::core::clone::Clone::clone(&self.height),
                    da_height: ::core::clone::Clone::clone(&self.da_height),
                    msg_receipt_count: ::core::clone::Clone::clone(
                        &self.msg_receipt_count,
                    ),
                    tx_root: ::core::clone::Clone::clone(&self.tx_root),
                    msg_receipt_root: ::core::clone::Clone::clone(
                        &self.msg_receipt_root,
                    ),
                    prev_id: ::core::clone::Clone::clone(&self.prev_id),
                    prev_root: ::core::clone::Clone::clone(&self.prev_root),
                    timestamp: ::core::clone::Clone::clone(&self.timestamp),
                    application_hash: ::core::clone::Clone::clone(
                        &self.application_hash,
                    ),
                    transactions: ::core::clone::Clone::clone(&self.transactions),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Block {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Block {
            #[inline]
            fn eq(&self, other: &Block) -> bool {
                self.id == other.id && self.height == other.height
                    && self.da_height == other.da_height
                    && self.msg_receipt_count == other.msg_receipt_count
                    && self.tx_root == other.tx_root
                    && self.msg_receipt_root == other.msg_receipt_root
                    && self.prev_id == other.prev_id && self.prev_root == other.prev_root
                    && self.timestamp == other.timestamp
                    && self.application_hash == other.application_hash
                    && self.transactions == other.transactions
            }
        }
        impl ::prost::Message for Block {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if self.id != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(1u32, &self.id, buf);
                }
                if self.height != 0u32 {
                    ::prost::encoding::uint32::encode(2u32, &self.height, buf);
                }
                if self.da_height != 0u64 {
                    ::prost::encoding::uint64::encode(3u32, &self.da_height, buf);
                }
                if self.msg_receipt_count != 0u64 {
                    ::prost::encoding::uint64::encode(
                        4u32,
                        &self.msg_receipt_count,
                        buf,
                    );
                }
                if self.tx_root != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(5u32, &self.tx_root, buf);
                }
                if self.msg_receipt_root != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(6u32, &self.msg_receipt_root, buf);
                }
                if self.prev_id != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(7u32, &self.prev_id, buf);
                }
                if self.prev_root != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(8u32, &self.prev_root, buf);
                }
                if self.timestamp != 0u64 {
                    ::prost::encoding::fixed64::encode(9u32, &self.timestamp, buf);
                }
                if self.application_hash != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(10u32, &self.application_hash, buf);
                }
                for msg in &self.transactions {
                    ::prost::encoding::message::encode(11u32, msg, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "Block";
                match tag {
                    1u32 => {
                        let mut value = &mut self.id;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "id");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.height;
                        ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "height");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.da_height;
                        ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "da_height");
                                error
                            })
                    }
                    4u32 => {
                        let mut value = &mut self.msg_receipt_count;
                        ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "msg_receipt_count");
                                error
                            })
                    }
                    5u32 => {
                        let mut value = &mut self.tx_root;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "tx_root");
                                error
                            })
                    }
                    6u32 => {
                        let mut value = &mut self.msg_receipt_root;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "msg_receipt_root");
                                error
                            })
                    }
                    7u32 => {
                        let mut value = &mut self.prev_id;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "prev_id");
                                error
                            })
                    }
                    8u32 => {
                        let mut value = &mut self.prev_root;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "prev_root");
                                error
                            })
                    }
                    9u32 => {
                        let mut value = &mut self.timestamp;
                        ::prost::encoding::fixed64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "timestamp");
                                error
                            })
                    }
                    10u32 => {
                        let mut value = &mut self.application_hash;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "application_hash");
                                error
                            })
                    }
                    11u32 => {
                        let mut value = &mut self.transactions;
                        ::prost::encoding::message::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "transactions");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.id != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(1u32, &self.id)
                    } else {
                        0
                    }
                    + if self.height != 0u32 {
                        ::prost::encoding::uint32::encoded_len(2u32, &self.height)
                    } else {
                        0
                    }
                    + if self.da_height != 0u64 {
                        ::prost::encoding::uint64::encoded_len(3u32, &self.da_height)
                    } else {
                        0
                    }
                    + if self.msg_receipt_count != 0u64 {
                        ::prost::encoding::uint64::encoded_len(
                            4u32,
                            &self.msg_receipt_count,
                        )
                    } else {
                        0
                    }
                    + if self.tx_root != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(5u32, &self.tx_root)
                    } else {
                        0
                    }
                    + if self.msg_receipt_root != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(
                            6u32,
                            &self.msg_receipt_root,
                        )
                    } else {
                        0
                    }
                    + if self.prev_id != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(7u32, &self.prev_id)
                    } else {
                        0
                    }
                    + if self.prev_root != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(8u32, &self.prev_root)
                    } else {
                        0
                    }
                    + if self.timestamp != 0u64 {
                        ::prost::encoding::fixed64::encoded_len(9u32, &self.timestamp)
                    } else {
                        0
                    }
                    + if self.application_hash != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(
                            10u32,
                            &self.application_hash,
                        )
                    } else {
                        0
                    }
                    + ::prost::encoding::message::encoded_len_repeated(
                        11u32,
                        &self.transactions,
                    )
            }
            fn clear(&mut self) {
                self.id.clear();
                self.height = 0u32;
                self.da_height = 0u64;
                self.msg_receipt_count = 0u64;
                self.tx_root.clear();
                self.msg_receipt_root.clear();
                self.prev_id.clear();
                self.prev_root.clear();
                self.timestamp = 0u64;
                self.application_hash.clear();
                self.transactions.clear();
            }
        }
        impl ::core::default::Default for Block {
            fn default() -> Self {
                Block {
                    id: ::core::default::Default::default(),
                    height: 0u32,
                    da_height: 0u64,
                    msg_receipt_count: 0u64,
                    tx_root: ::core::default::Default::default(),
                    msg_receipt_root: ::core::default::Default::default(),
                    prev_id: ::core::default::Default::default(),
                    prev_root: ::core::default::Default::default(),
                    timestamp: 0u64,
                    application_hash: ::core::default::Default::default(),
                    transactions: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for Block {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Block");
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.id)
                    };
                    builder.field("id", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.height)
                    };
                    builder.field("height", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.da_height)
                    };
                    builder.field("da_height", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.msg_receipt_count)
                    };
                    builder.field("msg_receipt_count", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.tx_root)
                    };
                    builder.field("tx_root", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.msg_receipt_root)
                    };
                    builder.field("msg_receipt_root", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.prev_id)
                    };
                    builder.field("prev_id", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.prev_root)
                    };
                    builder.field("prev_root", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.timestamp)
                    };
                    builder.field("timestamp", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.application_hash)
                    };
                    builder.field("application_hash", &wrapper)
                };
                let builder = {
                    let wrapper = &self.transactions;
                    builder.field("transactions", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        mod __block__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscBlock> for Block {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscBlock, graph::runtime::HostExportError> {
                    Ok(AscBlock {
                        id: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.id),
                            gas,
                        )?,
                        height: self.height,
                        da_height: self.da_height,
                        msg_receipt_count: self.msg_receipt_count,
                        tx_root: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.tx_root),
                            gas,
                        )?,
                        msg_receipt_root: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(
                                &self.msg_receipt_root,
                            ),
                            gas,
                        )?,
                        prev_id: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.prev_id),
                            gas,
                        )?,
                        prev_root: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.prev_root),
                            gas,
                        )?,
                        timestamp: self.timestamp,
                        application_hash: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(
                                &self.application_hash,
                            ),
                            gas,
                        )?,
                        transactions: graph::runtime::asc_new(
                            heap,
                            &self.transactions,
                            gas,
                        )?,
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscBlock {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelBlock;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscBlock {
            pub id: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub height: u32,
            pub da_height: u64,
            pub msg_receipt_count: u64,
            pub tx_root: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub msg_receipt_root: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub prev_id: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub prev_root: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub timestamp: u64,
            pub application_hash: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub transactions: graph::runtime::AscPtr<
                crate::protobuf::AscTransactionArray,
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscBlock {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "id",
                    "height",
                    "da_height",
                    "msg_receipt_count",
                    "tx_root",
                    "msg_receipt_root",
                    "prev_id",
                    "prev_root",
                    "timestamp",
                    "application_hash",
                    "transactions",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.id,
                    &&self.height,
                    &&self.da_height,
                    &&self.msg_receipt_count,
                    &&self.tx_root,
                    &&self.msg_receipt_root,
                    &&self.prev_id,
                    &&self.prev_root,
                    &&self.timestamp,
                    &&self.application_hash,
                    &&self.transactions,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "AscBlock",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscBlock {
            #[inline]
            fn default() -> AscBlock {
                AscBlock {
                    id: ::core::default::Default::default(),
                    height: ::core::default::Default::default(),
                    da_height: ::core::default::Default::default(),
                    msg_receipt_count: ::core::default::Default::default(),
                    tx_root: ::core::default::Default::default(),
                    msg_receipt_root: ::core::default::Default::default(),
                    prev_id: ::core::default::Default::default(),
                    prev_root: ::core::default::Default::default(),
                    timestamp: ::core::default::Default::default(),
                    application_hash: ::core::default::Default::default(),
                    transactions: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscBlock {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.id.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.height.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.da_height.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.msg_receipt_count.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.tx_root.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.msg_receipt_root.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.prev_id.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.prev_root.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.timestamp.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.application_hash.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscTransactionArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.transactions.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscBlock")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let id = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u32>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let height = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u64>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let da_height = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u64>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let msg_receipt_count = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let tx_root = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let msg_receipt_root = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let prev_id = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let prev_root = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u64>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let timestamp = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let application_hash = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscTransactionArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscTransactionArray>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let transactions = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self {
                    id,
                    height,
                    da_height,
                    msg_receipt_count,
                    tx_root,
                    msg_receipt_root,
                    prev_id,
                    prev_root,
                    timestamp,
                    application_hash,
                    transactions,
                })
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct Transaction {
            #[prost(oneof = "transaction::Kind", tags = "1, 2, 3")]
            pub kind: ::core::option::Option<transaction::Kind>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Transaction {
            #[inline]
            fn clone(&self) -> Transaction {
                Transaction {
                    kind: ::core::clone::Clone::clone(&self.kind),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Transaction {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Transaction {
            #[inline]
            fn eq(&self, other: &Transaction) -> bool {
                self.kind == other.kind
            }
        }
        impl ::prost::Message for Transaction {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if let Some(ref oneof) = self.kind {
                    oneof.encode(buf)
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "Transaction";
                match tag {
                    1u32 | 2u32 | 3u32 => {
                        let mut value = &mut self.kind;
                        transaction::Kind::merge(value, tag, wire_type, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "kind");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + self.kind.as_ref().map_or(0, transaction::Kind::encoded_len)
            }
            fn clear(&mut self) {
                self.kind = ::core::option::Option::None;
            }
        }
        impl ::core::default::Default for Transaction {
            fn default() -> Self {
                Transaction {
                    kind: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for Transaction {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Transaction");
                let builder = {
                    let wrapper = &self.kind;
                    builder.field("kind", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        pub struct AscTransactionArray(
            pub graph_runtime_wasm::asc_abi::class::Array<
                graph::runtime::AscPtr<AscTransaction>,
            >,
        );
        impl graph::runtime::ToAscObj<AscTransactionArray> for Vec<Transaction> {
            fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                &self,
                heap: &mut H,
                gas: &graph::runtime::gas::GasCounter,
            ) -> Result<AscTransactionArray, graph::runtime::HostExportError> {
                let content: Result<Vec<_>, _> = self
                    .iter()
                    .map(|x| graph::runtime::asc_new(heap, x, gas))
                    .collect();
                Ok(
                    AscTransactionArray(
                        graph_runtime_wasm::asc_abi::class::Array::new(
                            &content?,
                            heap,
                            gas,
                        )?,
                    ),
                )
            }
        }
        impl graph::runtime::AscType for AscTransactionArray {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                self.0.to_asc_bytes()
            }
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                Ok(
                    Self(
                        graph_runtime_wasm::asc_abi::class::Array::from_asc_bytes(
                            asc_obj,
                            api_version,
                        )?,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscTransactionArray {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelTransactionArray;
        }
        #[automatically_derived]
        mod __transaction__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscTransaction> for Transaction {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscTransaction, graph::runtime::HostExportError> {
                    let kind = self
                        .kind
                        .as_ref()
                        .ok_or_else(|| graph::runtime::HostExportError::from(
                            graph::runtime::DeterministicHostError::from(
                                ::anyhow::Error::msg({
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", " missing "],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(&"Transaction"),
                                                ::core::fmt::ArgumentV1::new_display(&"kind"),
                                            ],
                                        ),
                                    );
                                    res
                                }),
                            ),
                        ))?;
                    Ok(AscTransaction {
                        script: if let transaction::Kind::Script(v) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        create: if let transaction::Kind::Create(v) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        mint: if let transaction::Kind::Mint(v) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscTransaction {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelTransaction;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscTransaction {
            pub script: graph::runtime::AscPtr<AscScript>,
            pub create: graph::runtime::AscPtr<AscCreate>,
            pub mint: graph::runtime::AscPtr<AscMint>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscTransaction {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "AscTransaction",
                    "script",
                    &&self.script,
                    "create",
                    &&self.create,
                    "mint",
                    &&self.mint,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscTransaction {
            #[inline]
            fn default() -> AscTransaction {
                AscTransaction {
                    script: ::core::default::Default::default(),
                    create: ::core::default::Default::default(),
                    mint: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscTransaction {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscScript>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.script.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscCreate>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.create.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscMint>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.mint.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscTransaction")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscScript>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<AscScript>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let script = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscCreate>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<AscCreate>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let create = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscMint>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<graph::runtime::AscPtr<AscMint>>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let mint = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self { script, create, mint })
            }
        }
        /// Nested message and enum types in `Transaction`.
        pub mod transaction {
            #[allow(clippy::derive_partial_eq_without_eq)]
            pub enum Kind {
                #[prost(message, tag = "1")]
                Script(super::Script),
                #[prost(message, tag = "2")]
                Create(super::Create),
                #[prost(message, tag = "3")]
                Mint(super::Mint),
            }
            #[automatically_derived]
            #[allow(clippy::derive_partial_eq_without_eq)]
            impl ::core::clone::Clone for Kind {
                #[inline]
                fn clone(&self) -> Kind {
                    match self {
                        Kind::Script(__self_0) => {
                            Kind::Script(::core::clone::Clone::clone(__self_0))
                        }
                        Kind::Create(__self_0) => {
                            Kind::Create(::core::clone::Clone::clone(__self_0))
                        }
                        Kind::Mint(__self_0) => {
                            Kind::Mint(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Kind {}
            #[automatically_derived]
            #[allow(clippy::derive_partial_eq_without_eq)]
            impl ::core::cmp::PartialEq for Kind {
                #[inline]
                fn eq(&self, other: &Kind) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                        && match (self, other) {
                            (Kind::Script(__self_0), Kind::Script(__arg1_0)) => {
                                *__self_0 == *__arg1_0
                            }
                            (Kind::Create(__self_0), Kind::Create(__arg1_0)) => {
                                *__self_0 == *__arg1_0
                            }
                            (Kind::Mint(__self_0), Kind::Mint(__arg1_0)) => {
                                *__self_0 == *__arg1_0
                            }
                            _ => unsafe { ::core::intrinsics::unreachable() }
                        }
                }
            }
            impl Kind {
                /// Encodes the message to a buffer.
                pub fn encode<B>(&self, buf: &mut B)
                where
                    B: ::prost::bytes::BufMut,
                {
                    match *self {
                        Kind::Script(ref value) => {
                            ::prost::encoding::message::encode(1u32, &*value, buf);
                        }
                        Kind::Create(ref value) => {
                            ::prost::encoding::message::encode(2u32, &*value, buf);
                        }
                        Kind::Mint(ref value) => {
                            ::prost::encoding::message::encode(3u32, &*value, buf);
                        }
                    }
                }
                /// Decodes an instance of the message from a buffer, and merges it into self.
                pub fn merge<B>(
                    field: &mut ::core::option::Option<Kind>,
                    tag: u32,
                    wire_type: ::prost::encoding::WireType,
                    buf: &mut B,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError>
                where
                    B: ::prost::bytes::Buf,
                {
                    match tag {
                        1u32 => {
                            match field {
                                ::core::option::Option::Some(
                                    Kind::Script(ref mut value),
                                ) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::Script(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        2u32 => {
                            match field {
                                ::core::option::Option::Some(
                                    Kind::Create(ref mut value),
                                ) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::Create(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        3u32 => {
                            match field {
                                ::core::option::Option::Some(Kind::Mint(ref mut value)) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::Mint(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["internal error: entered unreachable code: "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(
                                            &::core::fmt::Arguments::new_v1(
                                                &["invalid Kind tag: "],
                                                &[::core::fmt::ArgumentV1::new_display(&tag)],
                                            ),
                                        ),
                                    ],
                                ),
                            )
                        }
                    }
                }
                /// Returns the encoded length of the message without a length delimiter.
                #[inline]
                pub fn encoded_len(&self) -> usize {
                    match *self {
                        Kind::Script(ref value) => {
                            ::prost::encoding::message::encoded_len(1u32, &*value)
                        }
                        Kind::Create(ref value) => {
                            ::prost::encoding::message::encoded_len(2u32, &*value)
                        }
                        Kind::Mint(ref value) => {
                            ::prost::encoding::message::encoded_len(3u32, &*value)
                        }
                    }
                }
            }
            impl ::core::fmt::Debug for Kind {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Kind::Script(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("Script").field(&wrapper).finish()
                        }
                        Kind::Create(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("Create").field(&wrapper).finish()
                        }
                        Kind::Mint(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("Mint").field(&wrapper).finish()
                        }
                    }
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct Script {
            #[prost(uint64, tag = "1")]
            pub script_gas_limit: u64,
            #[prost(bytes = "vec", tag = "2")]
            pub script: ::prost::alloc::vec::Vec<u8>,
            #[prost(bytes = "vec", tag = "3")]
            pub script_data: ::prost::alloc::vec::Vec<u8>,
            #[prost(message, optional, tag = "4")]
            pub policies: ::core::option::Option<Policies>,
            #[prost(message, repeated, tag = "5")]
            pub inputs: ::prost::alloc::vec::Vec<Input>,
            #[prost(message, repeated, tag = "6")]
            pub outputs: ::prost::alloc::vec::Vec<Output>,
            #[prost(bytes = "vec", repeated, tag = "7")]
            pub witnesses: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
            #[prost(bytes = "vec", tag = "8")]
            pub receipts_root: ::prost::alloc::vec::Vec<u8>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Script {
            #[inline]
            fn clone(&self) -> Script {
                Script {
                    script_gas_limit: ::core::clone::Clone::clone(
                        &self.script_gas_limit,
                    ),
                    script: ::core::clone::Clone::clone(&self.script),
                    script_data: ::core::clone::Clone::clone(&self.script_data),
                    policies: ::core::clone::Clone::clone(&self.policies),
                    inputs: ::core::clone::Clone::clone(&self.inputs),
                    outputs: ::core::clone::Clone::clone(&self.outputs),
                    witnesses: ::core::clone::Clone::clone(&self.witnesses),
                    receipts_root: ::core::clone::Clone::clone(&self.receipts_root),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Script {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Script {
            #[inline]
            fn eq(&self, other: &Script) -> bool {
                self.script_gas_limit == other.script_gas_limit
                    && self.script == other.script
                    && self.script_data == other.script_data
                    && self.policies == other.policies && self.inputs == other.inputs
                    && self.outputs == other.outputs && self.witnesses == other.witnesses
                    && self.receipts_root == other.receipts_root
            }
        }
        impl ::prost::Message for Script {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if self.script_gas_limit != 0u64 {
                    ::prost::encoding::uint64::encode(1u32, &self.script_gas_limit, buf);
                }
                if self.script != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(2u32, &self.script, buf);
                }
                if self.script_data != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(3u32, &self.script_data, buf);
                }
                if let Some(ref msg) = self.policies {
                    ::prost::encoding::message::encode(4u32, msg, buf);
                }
                for msg in &self.inputs {
                    ::prost::encoding::message::encode(5u32, msg, buf);
                }
                for msg in &self.outputs {
                    ::prost::encoding::message::encode(6u32, msg, buf);
                }
                ::prost::encoding::bytes::encode_repeated(7u32, &self.witnesses, buf);
                if self.receipts_root != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(8u32, &self.receipts_root, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "Script";
                match tag {
                    1u32 => {
                        let mut value = &mut self.script_gas_limit;
                        ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "script_gas_limit");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.script;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "script");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.script_data;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "script_data");
                                error
                            })
                    }
                    4u32 => {
                        let mut value = &mut self.policies;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "policies");
                                error
                            })
                    }
                    5u32 => {
                        let mut value = &mut self.inputs;
                        ::prost::encoding::message::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "inputs");
                                error
                            })
                    }
                    6u32 => {
                        let mut value = &mut self.outputs;
                        ::prost::encoding::message::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "outputs");
                                error
                            })
                    }
                    7u32 => {
                        let mut value = &mut self.witnesses;
                        ::prost::encoding::bytes::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "witnesses");
                                error
                            })
                    }
                    8u32 => {
                        let mut value = &mut self.receipts_root;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "receipts_root");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.script_gas_limit != 0u64 {
                        ::prost::encoding::uint64::encoded_len(
                            1u32,
                            &self.script_gas_limit,
                        )
                    } else {
                        0
                    }
                    + if self.script != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(2u32, &self.script)
                    } else {
                        0
                    }
                    + if self.script_data != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(3u32, &self.script_data)
                    } else {
                        0
                    }
                    + self
                        .policies
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(4u32, msg),
                        )
                    + ::prost::encoding::message::encoded_len_repeated(
                        5u32,
                        &self.inputs,
                    )
                    + ::prost::encoding::message::encoded_len_repeated(
                        6u32,
                        &self.outputs,
                    )
                    + ::prost::encoding::bytes::encoded_len_repeated(
                        7u32,
                        &self.witnesses,
                    )
                    + if self.receipts_root != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(8u32, &self.receipts_root)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.script_gas_limit = 0u64;
                self.script.clear();
                self.script_data.clear();
                self.policies = ::core::option::Option::None;
                self.inputs.clear();
                self.outputs.clear();
                self.witnesses.clear();
                self.receipts_root.clear();
            }
        }
        impl ::core::default::Default for Script {
            fn default() -> Self {
                Script {
                    script_gas_limit: 0u64,
                    script: ::core::default::Default::default(),
                    script_data: ::core::default::Default::default(),
                    policies: ::core::default::Default::default(),
                    inputs: ::core::default::Default::default(),
                    outputs: ::core::default::Default::default(),
                    witnesses: ::prost::alloc::vec::Vec::new(),
                    receipts_root: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for Script {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Script");
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.script_gas_limit)
                    };
                    builder.field("script_gas_limit", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.script)
                    };
                    builder.field("script", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.script_data)
                    };
                    builder.field("script_data", &wrapper)
                };
                let builder = {
                    let wrapper = &self.policies;
                    builder.field("policies", &wrapper)
                };
                let builder = {
                    let wrapper = &self.inputs;
                    builder.field("inputs", &wrapper)
                };
                let builder = {
                    let wrapper = &self.outputs;
                    builder.field("outputs", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(
                            &'a ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
                        );
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                let mut vec_builder = f.debug_list();
                                for v in self.0 {
                                    fn Inner<T>(v: T) -> T {
                                        v
                                    }
                                    vec_builder.entry(&Inner(v));
                                }
                                vec_builder.finish()
                            }
                        }
                        ScalarWrapper(&self.witnesses)
                    };
                    builder.field("witnesses", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.receipts_root)
                    };
                    builder.field("receipts_root", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        mod __script__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscScript> for Script {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscScript, graph::runtime::HostExportError> {
                    Ok(AscScript {
                        script_gas_limit: self.script_gas_limit,
                        script: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.script),
                            gas,
                        )?,
                        script_data: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(
                                &self.script_data,
                            ),
                            gas,
                        )?,
                        policies: graph::runtime::asc_new_or_null(
                            heap,
                            &self.policies,
                            gas,
                        )?,
                        inputs: graph::runtime::asc_new(heap, &self.inputs, gas)?,
                        outputs: graph::runtime::asc_new(heap, &self.outputs, gas)?,
                        witnesses: graph::runtime::asc_new(heap, &self.witnesses, gas)?,
                        receipts_root: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(
                                &self.receipts_root,
                            ),
                            gas,
                        )?,
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscScript {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelScript;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscScript {
            pub script_gas_limit: u64,
            pub script: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub script_data: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub policies: graph::runtime::AscPtr<crate::protobuf::AscPolicies>,
            pub inputs: graph::runtime::AscPtr<crate::protobuf::AscInputArray>,
            pub outputs: graph::runtime::AscPtr<crate::protobuf::AscOutputArray>,
            pub witnesses: graph::runtime::AscPtr<crate::protobuf::AscBytesArray>,
            pub receipts_root: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscScript {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "script_gas_limit",
                    "script",
                    "script_data",
                    "policies",
                    "inputs",
                    "outputs",
                    "witnesses",
                    "receipts_root",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.script_gas_limit,
                    &&self.script,
                    &&self.script_data,
                    &&self.policies,
                    &&self.inputs,
                    &&self.outputs,
                    &&self.witnesses,
                    &&self.receipts_root,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "AscScript",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscScript {
            #[inline]
            fn default() -> AscScript {
                AscScript {
                    script_gas_limit: ::core::default::Default::default(),
                    script: ::core::default::Default::default(),
                    script_data: ::core::default::Default::default(),
                    policies: ::core::default::Default::default(),
                    inputs: ::core::default::Default::default(),
                    outputs: ::core::default::Default::default(),
                    witnesses: ::core::default::Default::default(),
                    receipts_root: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscScript {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.script_gas_limit.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.script.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.script_data.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscPolicies>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.policies.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscInputArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.inputs.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscOutputArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.outputs.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscBytesArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.witnesses.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.receipts_root.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscScript")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u64>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let script_gas_limit = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let script = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let script_data = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscPolicies>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscPolicies>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let policies = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscInputArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscInputArray>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let inputs = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscOutputArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscOutputArray>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let outputs = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscBytesArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscBytesArray>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let witnesses = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let receipts_root = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self {
                    script_gas_limit,
                    script,
                    script_data,
                    policies,
                    inputs,
                    outputs,
                    witnesses,
                    receipts_root,
                })
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct Create {
            #[prost(uint64, tag = "1")]
            pub bytecode_length: u64,
            #[prost(uint32, tag = "2")]
            pub bytecode_witness_index: u32,
            #[prost(message, optional, tag = "3")]
            pub policies: ::core::option::Option<Policies>,
            #[prost(message, repeated, tag = "4")]
            pub storage_slots: ::prost::alloc::vec::Vec<StorageSlot>,
            #[prost(message, repeated, tag = "5")]
            pub inputs: ::prost::alloc::vec::Vec<Input>,
            #[prost(message, repeated, tag = "6")]
            pub outputs: ::prost::alloc::vec::Vec<Output>,
            #[prost(bytes = "vec", repeated, tag = "7")]
            pub witnesses: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
            #[prost(bytes = "vec", tag = "8")]
            pub salt: ::prost::alloc::vec::Vec<u8>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Create {
            #[inline]
            fn clone(&self) -> Create {
                Create {
                    bytecode_length: ::core::clone::Clone::clone(&self.bytecode_length),
                    bytecode_witness_index: ::core::clone::Clone::clone(
                        &self.bytecode_witness_index,
                    ),
                    policies: ::core::clone::Clone::clone(&self.policies),
                    storage_slots: ::core::clone::Clone::clone(&self.storage_slots),
                    inputs: ::core::clone::Clone::clone(&self.inputs),
                    outputs: ::core::clone::Clone::clone(&self.outputs),
                    witnesses: ::core::clone::Clone::clone(&self.witnesses),
                    salt: ::core::clone::Clone::clone(&self.salt),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Create {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Create {
            #[inline]
            fn eq(&self, other: &Create) -> bool {
                self.bytecode_length == other.bytecode_length
                    && self.bytecode_witness_index == other.bytecode_witness_index
                    && self.policies == other.policies
                    && self.storage_slots == other.storage_slots
                    && self.inputs == other.inputs && self.outputs == other.outputs
                    && self.witnesses == other.witnesses && self.salt == other.salt
            }
        }
        impl ::prost::Message for Create {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if self.bytecode_length != 0u64 {
                    ::prost::encoding::uint64::encode(1u32, &self.bytecode_length, buf);
                }
                if self.bytecode_witness_index != 0u32 {
                    ::prost::encoding::uint32::encode(
                        2u32,
                        &self.bytecode_witness_index,
                        buf,
                    );
                }
                if let Some(ref msg) = self.policies {
                    ::prost::encoding::message::encode(3u32, msg, buf);
                }
                for msg in &self.storage_slots {
                    ::prost::encoding::message::encode(4u32, msg, buf);
                }
                for msg in &self.inputs {
                    ::prost::encoding::message::encode(5u32, msg, buf);
                }
                for msg in &self.outputs {
                    ::prost::encoding::message::encode(6u32, msg, buf);
                }
                ::prost::encoding::bytes::encode_repeated(7u32, &self.witnesses, buf);
                if self.salt != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(8u32, &self.salt, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "Create";
                match tag {
                    1u32 => {
                        let mut value = &mut self.bytecode_length;
                        ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "bytecode_length");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.bytecode_witness_index;
                        ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "bytecode_witness_index");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.policies;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "policies");
                                error
                            })
                    }
                    4u32 => {
                        let mut value = &mut self.storage_slots;
                        ::prost::encoding::message::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "storage_slots");
                                error
                            })
                    }
                    5u32 => {
                        let mut value = &mut self.inputs;
                        ::prost::encoding::message::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "inputs");
                                error
                            })
                    }
                    6u32 => {
                        let mut value = &mut self.outputs;
                        ::prost::encoding::message::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "outputs");
                                error
                            })
                    }
                    7u32 => {
                        let mut value = &mut self.witnesses;
                        ::prost::encoding::bytes::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "witnesses");
                                error
                            })
                    }
                    8u32 => {
                        let mut value = &mut self.salt;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "salt");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.bytecode_length != 0u64 {
                        ::prost::encoding::uint64::encoded_len(
                            1u32,
                            &self.bytecode_length,
                        )
                    } else {
                        0
                    }
                    + if self.bytecode_witness_index != 0u32 {
                        ::prost::encoding::uint32::encoded_len(
                            2u32,
                            &self.bytecode_witness_index,
                        )
                    } else {
                        0
                    }
                    + self
                        .policies
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(3u32, msg),
                        )
                    + ::prost::encoding::message::encoded_len_repeated(
                        4u32,
                        &self.storage_slots,
                    )
                    + ::prost::encoding::message::encoded_len_repeated(
                        5u32,
                        &self.inputs,
                    )
                    + ::prost::encoding::message::encoded_len_repeated(
                        6u32,
                        &self.outputs,
                    )
                    + ::prost::encoding::bytes::encoded_len_repeated(
                        7u32,
                        &self.witnesses,
                    )
                    + if self.salt != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(8u32, &self.salt)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.bytecode_length = 0u64;
                self.bytecode_witness_index = 0u32;
                self.policies = ::core::option::Option::None;
                self.storage_slots.clear();
                self.inputs.clear();
                self.outputs.clear();
                self.witnesses.clear();
                self.salt.clear();
            }
        }
        impl ::core::default::Default for Create {
            fn default() -> Self {
                Create {
                    bytecode_length: 0u64,
                    bytecode_witness_index: 0u32,
                    policies: ::core::default::Default::default(),
                    storage_slots: ::core::default::Default::default(),
                    inputs: ::core::default::Default::default(),
                    outputs: ::core::default::Default::default(),
                    witnesses: ::prost::alloc::vec::Vec::new(),
                    salt: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for Create {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Create");
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.bytecode_length)
                    };
                    builder.field("bytecode_length", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.bytecode_witness_index)
                    };
                    builder.field("bytecode_witness_index", &wrapper)
                };
                let builder = {
                    let wrapper = &self.policies;
                    builder.field("policies", &wrapper)
                };
                let builder = {
                    let wrapper = &self.storage_slots;
                    builder.field("storage_slots", &wrapper)
                };
                let builder = {
                    let wrapper = &self.inputs;
                    builder.field("inputs", &wrapper)
                };
                let builder = {
                    let wrapper = &self.outputs;
                    builder.field("outputs", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(
                            &'a ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
                        );
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                let mut vec_builder = f.debug_list();
                                for v in self.0 {
                                    fn Inner<T>(v: T) -> T {
                                        v
                                    }
                                    vec_builder.entry(&Inner(v));
                                }
                                vec_builder.finish()
                            }
                        }
                        ScalarWrapper(&self.witnesses)
                    };
                    builder.field("witnesses", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.salt)
                    };
                    builder.field("salt", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        mod __create__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscCreate> for Create {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscCreate, graph::runtime::HostExportError> {
                    Ok(AscCreate {
                        bytecode_length: self.bytecode_length,
                        bytecode_witness_index: self.bytecode_witness_index,
                        policies: graph::runtime::asc_new_or_null(
                            heap,
                            &self.policies,
                            gas,
                        )?,
                        storage_slots: graph::runtime::asc_new(
                            heap,
                            &self.storage_slots,
                            gas,
                        )?,
                        inputs: graph::runtime::asc_new(heap, &self.inputs, gas)?,
                        outputs: graph::runtime::asc_new(heap, &self.outputs, gas)?,
                        witnesses: graph::runtime::asc_new(heap, &self.witnesses, gas)?,
                        salt: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.salt),
                            gas,
                        )?,
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscCreate {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelCreate;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscCreate {
            pub bytecode_length: u64,
            pub bytecode_witness_index: u32,
            pub policies: graph::runtime::AscPtr<crate::protobuf::AscPolicies>,
            pub storage_slots: graph::runtime::AscPtr<
                crate::protobuf::AscStorageSlotArray,
            >,
            pub inputs: graph::runtime::AscPtr<crate::protobuf::AscInputArray>,
            pub outputs: graph::runtime::AscPtr<crate::protobuf::AscOutputArray>,
            pub witnesses: graph::runtime::AscPtr<crate::protobuf::AscBytesArray>,
            pub salt: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscCreate {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "bytecode_length",
                    "bytecode_witness_index",
                    "policies",
                    "storage_slots",
                    "inputs",
                    "outputs",
                    "witnesses",
                    "salt",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.bytecode_length,
                    &&self.bytecode_witness_index,
                    &&self.policies,
                    &&self.storage_slots,
                    &&self.inputs,
                    &&self.outputs,
                    &&self.witnesses,
                    &&self.salt,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "AscCreate",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscCreate {
            #[inline]
            fn default() -> AscCreate {
                AscCreate {
                    bytecode_length: ::core::default::Default::default(),
                    bytecode_witness_index: ::core::default::Default::default(),
                    policies: ::core::default::Default::default(),
                    storage_slots: ::core::default::Default::default(),
                    inputs: ::core::default::Default::default(),
                    outputs: ::core::default::Default::default(),
                    witnesses: ::core::default::Default::default(),
                    salt: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscCreate {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.bytecode_length.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.bytecode_witness_index.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscPolicies>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.policies.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscStorageSlotArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.storage_slots.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscInputArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.inputs.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscOutputArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.outputs.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscBytesArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.witnesses.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.salt.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscCreate")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u64>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let bytecode_length = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u32>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let bytecode_witness_index = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscPolicies>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscPolicies>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let policies = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscStorageSlotArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscStorageSlotArray>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let storage_slots = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscInputArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscInputArray>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let inputs = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscOutputArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscOutputArray>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let outputs = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscBytesArray>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscBytesArray>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let witnesses = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let salt = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self {
                    bytecode_length,
                    bytecode_witness_index,
                    policies,
                    storage_slots,
                    inputs,
                    outputs,
                    witnesses,
                    salt,
                })
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct Mint {
            #[prost(message, optional, tag = "1")]
            pub tx_pointer: ::core::option::Option<TxPointer>,
            #[prost(message, optional, tag = "2")]
            pub input_contract: ::core::option::Option<InputContract>,
            #[prost(message, optional, tag = "3")]
            pub output_contract: ::core::option::Option<OutputContract>,
            #[prost(uint64, tag = "4")]
            pub mint_amount: u64,
            #[prost(bytes = "vec", tag = "5")]
            pub mint_asset_id: ::prost::alloc::vec::Vec<u8>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Mint {
            #[inline]
            fn clone(&self) -> Mint {
                Mint {
                    tx_pointer: ::core::clone::Clone::clone(&self.tx_pointer),
                    input_contract: ::core::clone::Clone::clone(&self.input_contract),
                    output_contract: ::core::clone::Clone::clone(&self.output_contract),
                    mint_amount: ::core::clone::Clone::clone(&self.mint_amount),
                    mint_asset_id: ::core::clone::Clone::clone(&self.mint_asset_id),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Mint {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Mint {
            #[inline]
            fn eq(&self, other: &Mint) -> bool {
                self.tx_pointer == other.tx_pointer
                    && self.input_contract == other.input_contract
                    && self.output_contract == other.output_contract
                    && self.mint_amount == other.mint_amount
                    && self.mint_asset_id == other.mint_asset_id
            }
        }
        impl ::prost::Message for Mint {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if let Some(ref msg) = self.tx_pointer {
                    ::prost::encoding::message::encode(1u32, msg, buf);
                }
                if let Some(ref msg) = self.input_contract {
                    ::prost::encoding::message::encode(2u32, msg, buf);
                }
                if let Some(ref msg) = self.output_contract {
                    ::prost::encoding::message::encode(3u32, msg, buf);
                }
                if self.mint_amount != 0u64 {
                    ::prost::encoding::uint64::encode(4u32, &self.mint_amount, buf);
                }
                if self.mint_asset_id != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(5u32, &self.mint_asset_id, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "Mint";
                match tag {
                    1u32 => {
                        let mut value = &mut self.tx_pointer;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "tx_pointer");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.input_contract;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "input_contract");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.output_contract;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "output_contract");
                                error
                            })
                    }
                    4u32 => {
                        let mut value = &mut self.mint_amount;
                        ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "mint_amount");
                                error
                            })
                    }
                    5u32 => {
                        let mut value = &mut self.mint_asset_id;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "mint_asset_id");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + self
                        .tx_pointer
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                        )
                    + self
                        .input_contract
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(2u32, msg),
                        )
                    + self
                        .output_contract
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(3u32, msg),
                        )
                    + if self.mint_amount != 0u64 {
                        ::prost::encoding::uint64::encoded_len(4u32, &self.mint_amount)
                    } else {
                        0
                    }
                    + if self.mint_asset_id != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(5u32, &self.mint_asset_id)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.tx_pointer = ::core::option::Option::None;
                self.input_contract = ::core::option::Option::None;
                self.output_contract = ::core::option::Option::None;
                self.mint_amount = 0u64;
                self.mint_asset_id.clear();
            }
        }
        impl ::core::default::Default for Mint {
            fn default() -> Self {
                Mint {
                    tx_pointer: ::core::default::Default::default(),
                    input_contract: ::core::default::Default::default(),
                    output_contract: ::core::default::Default::default(),
                    mint_amount: 0u64,
                    mint_asset_id: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for Mint {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Mint");
                let builder = {
                    let wrapper = &self.tx_pointer;
                    builder.field("tx_pointer", &wrapper)
                };
                let builder = {
                    let wrapper = &self.input_contract;
                    builder.field("input_contract", &wrapper)
                };
                let builder = {
                    let wrapper = &self.output_contract;
                    builder.field("output_contract", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.mint_amount)
                    };
                    builder.field("mint_amount", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.mint_asset_id)
                    };
                    builder.field("mint_asset_id", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        mod __mint__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscMint> for Mint {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscMint, graph::runtime::HostExportError> {
                    Ok(AscMint {
                        tx_pointer: graph::runtime::asc_new_or_null(
                            heap,
                            &self.tx_pointer,
                            gas,
                        )?,
                        input_contract: graph::runtime::asc_new_or_null(
                            heap,
                            &self.input_contract,
                            gas,
                        )?,
                        output_contract: graph::runtime::asc_new_or_null(
                            heap,
                            &self.output_contract,
                            gas,
                        )?,
                        mint_amount: self.mint_amount,
                        mint_asset_id: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(
                                &self.mint_asset_id,
                            ),
                            gas,
                        )?,
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscMint {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelMint;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscMint {
            pub tx_pointer: graph::runtime::AscPtr<crate::protobuf::AscTxPointer>,
            pub input_contract: graph::runtime::AscPtr<
                crate::protobuf::AscInputContract,
            >,
            pub output_contract: graph::runtime::AscPtr<
                crate::protobuf::AscOutputContract,
            >,
            pub mint_amount: u64,
            pub mint_asset_id: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscMint {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "AscMint",
                    "tx_pointer",
                    &&self.tx_pointer,
                    "input_contract",
                    &&self.input_contract,
                    "output_contract",
                    &&self.output_contract,
                    "mint_amount",
                    &&self.mint_amount,
                    "mint_asset_id",
                    &&self.mint_asset_id,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscMint {
            #[inline]
            fn default() -> AscMint {
                AscMint {
                    tx_pointer: ::core::default::Default::default(),
                    input_contract: ::core::default::Default::default(),
                    output_contract: ::core::default::Default::default(),
                    mint_amount: ::core::default::Default::default(),
                    mint_asset_id: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscMint {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscTxPointer>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.tx_pointer.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscInputContract>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.input_contract.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscOutputContract>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.output_contract.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.mint_amount.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.mint_asset_id.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscMint")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscTxPointer>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscTxPointer>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let tx_pointer = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscInputContract>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscInputContract>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let input_contract = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscOutputContract>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscOutputContract>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let output_contract = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u64>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let mint_amount = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let mint_asset_id = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self {
                    tx_pointer,
                    input_contract,
                    output_contract,
                    mint_amount,
                    mint_asset_id,
                })
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct Input {
            #[prost(oneof = "input::Kind", tags = "1, 2, 3, 4, 5, 6, 7")]
            pub kind: ::core::option::Option<input::Kind>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Input {
            #[inline]
            fn clone(&self) -> Input {
                Input {
                    kind: ::core::clone::Clone::clone(&self.kind),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Input {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Input {
            #[inline]
            fn eq(&self, other: &Input) -> bool {
                self.kind == other.kind
            }
        }
        impl ::prost::Message for Input {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if let Some(ref oneof) = self.kind {
                    oneof.encode(buf)
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "Input";
                match tag {
                    1u32 | 2u32 | 3u32 | 4u32 | 5u32 | 6u32 | 7u32 => {
                        let mut value = &mut self.kind;
                        input::Kind::merge(value, tag, wire_type, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "kind");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + self.kind.as_ref().map_or(0, input::Kind::encoded_len)
            }
            fn clear(&mut self) {
                self.kind = ::core::option::Option::None;
            }
        }
        impl ::core::default::Default for Input {
            fn default() -> Self {
                Input {
                    kind: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for Input {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Input");
                let builder = {
                    let wrapper = &self.kind;
                    builder.field("kind", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        pub struct AscInputArray(
            pub graph_runtime_wasm::asc_abi::class::Array<
                graph::runtime::AscPtr<AscInput>,
            >,
        );
        impl graph::runtime::ToAscObj<AscInputArray> for Vec<Input> {
            fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                &self,
                heap: &mut H,
                gas: &graph::runtime::gas::GasCounter,
            ) -> Result<AscInputArray, graph::runtime::HostExportError> {
                let content: Result<Vec<_>, _> = self
                    .iter()
                    .map(|x| graph::runtime::asc_new(heap, x, gas))
                    .collect();
                Ok(
                    AscInputArray(
                        graph_runtime_wasm::asc_abi::class::Array::new(
                            &content?,
                            heap,
                            gas,
                        )?,
                    ),
                )
            }
        }
        impl graph::runtime::AscType for AscInputArray {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                self.0.to_asc_bytes()
            }
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                Ok(
                    Self(
                        graph_runtime_wasm::asc_abi::class::Array::from_asc_bytes(
                            asc_obj,
                            api_version,
                        )?,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscInputArray {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelInputArray;
        }
        #[automatically_derived]
        mod __input__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscInput> for Input {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscInput, graph::runtime::HostExportError> {
                    let kind = self
                        .kind
                        .as_ref()
                        .ok_or_else(|| graph::runtime::HostExportError::from(
                            graph::runtime::DeterministicHostError::from(
                                ::anyhow::Error::msg({
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", " missing "],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(&"Input"),
                                                ::core::fmt::ArgumentV1::new_display(&"kind"),
                                            ],
                                        ),
                                    );
                                    res
                                }),
                            ),
                        ))?;
                    Ok(AscInput {
                        coin_signed: if let input::Kind::CoinSigned(v) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        coin_predicate: if let input::Kind::CoinPredicate(v) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        contract: if let input::Kind::Contract(v) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        message_coin_signed: if let input::Kind::MessageCoinSigned(v) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        message_coin_predicate: if let input::Kind::MessageCoinPredicate(
                            v,
                        ) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        message_data_signed: if let input::Kind::MessageDataSigned(v) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        message_data_predicate: if let input::Kind::MessageDataPredicate(
                            v,
                        ) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscInput {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelInput;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscInput {
            pub coin_signed: graph::runtime::AscPtr<AscCoin>,
            pub coin_predicate: graph::runtime::AscPtr<AscCoin>,
            pub contract: graph::runtime::AscPtr<AscInputContract>,
            pub message_coin_signed: graph::runtime::AscPtr<AscMessage>,
            pub message_coin_predicate: graph::runtime::AscPtr<AscMessage>,
            pub message_data_signed: graph::runtime::AscPtr<AscMessage>,
            pub message_data_predicate: graph::runtime::AscPtr<AscMessage>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscInput {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "coin_signed",
                    "coin_predicate",
                    "contract",
                    "message_coin_signed",
                    "message_coin_predicate",
                    "message_data_signed",
                    "message_data_predicate",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.coin_signed,
                    &&self.coin_predicate,
                    &&self.contract,
                    &&self.message_coin_signed,
                    &&self.message_coin_predicate,
                    &&self.message_data_signed,
                    &&self.message_data_predicate,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "AscInput",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscInput {
            #[inline]
            fn default() -> AscInput {
                AscInput {
                    coin_signed: ::core::default::Default::default(),
                    coin_predicate: ::core::default::Default::default(),
                    contract: ::core::default::Default::default(),
                    message_coin_signed: ::core::default::Default::default(),
                    message_coin_predicate: ::core::default::Default::default(),
                    message_data_signed: ::core::default::Default::default(),
                    message_data_predicate: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscInput {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscCoin>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.coin_signed.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscCoin>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.coin_predicate.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscInputContract>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.contract.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscMessage>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.message_coin_signed.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscMessage>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.message_coin_predicate.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscMessage>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.message_data_signed.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscMessage>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.message_data_predicate.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscInput")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscCoin>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<graph::runtime::AscPtr<AscCoin>>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let coin_signed = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscCoin>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<graph::runtime::AscPtr<AscCoin>>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let coin_predicate = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscInputContract>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<AscInputContract>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let contract = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscMessage>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<AscMessage>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let message_coin_signed = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscMessage>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<AscMessage>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let message_coin_predicate = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscMessage>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<AscMessage>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let message_data_signed = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscMessage>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<AscMessage>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let message_data_predicate = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self {
                    coin_signed,
                    coin_predicate,
                    contract,
                    message_coin_signed,
                    message_coin_predicate,
                    message_data_signed,
                    message_data_predicate,
                })
            }
        }
        /// Nested message and enum types in `Input`.
        pub mod input {
            #[allow(clippy::derive_partial_eq_without_eq)]
            pub enum Kind {
                #[prost(message, tag = "1")]
                CoinSigned(super::Coin),
                #[prost(message, tag = "2")]
                CoinPredicate(super::Coin),
                #[prost(message, tag = "3")]
                Contract(super::InputContract),
                #[prost(message, tag = "4")]
                MessageCoinSigned(super::Message),
                #[prost(message, tag = "5")]
                MessageCoinPredicate(super::Message),
                #[prost(message, tag = "6")]
                MessageDataSigned(super::Message),
                #[prost(message, tag = "7")]
                MessageDataPredicate(super::Message),
            }
            #[automatically_derived]
            #[allow(clippy::derive_partial_eq_without_eq)]
            impl ::core::clone::Clone for Kind {
                #[inline]
                fn clone(&self) -> Kind {
                    match self {
                        Kind::CoinSigned(__self_0) => {
                            Kind::CoinSigned(::core::clone::Clone::clone(__self_0))
                        }
                        Kind::CoinPredicate(__self_0) => {
                            Kind::CoinPredicate(::core::clone::Clone::clone(__self_0))
                        }
                        Kind::Contract(__self_0) => {
                            Kind::Contract(::core::clone::Clone::clone(__self_0))
                        }
                        Kind::MessageCoinSigned(__self_0) => {
                            Kind::MessageCoinSigned(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        Kind::MessageCoinPredicate(__self_0) => {
                            Kind::MessageCoinPredicate(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        Kind::MessageDataSigned(__self_0) => {
                            Kind::MessageDataSigned(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        Kind::MessageDataPredicate(__self_0) => {
                            Kind::MessageDataPredicate(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                    }
                }
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Kind {}
            #[automatically_derived]
            #[allow(clippy::derive_partial_eq_without_eq)]
            impl ::core::cmp::PartialEq for Kind {
                #[inline]
                fn eq(&self, other: &Kind) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                        && match (self, other) {
                            (Kind::CoinSigned(__self_0), Kind::CoinSigned(__arg1_0)) => {
                                *__self_0 == *__arg1_0
                            }
                            (
                                Kind::CoinPredicate(__self_0),
                                Kind::CoinPredicate(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (Kind::Contract(__self_0), Kind::Contract(__arg1_0)) => {
                                *__self_0 == *__arg1_0
                            }
                            (
                                Kind::MessageCoinSigned(__self_0),
                                Kind::MessageCoinSigned(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                Kind::MessageCoinPredicate(__self_0),
                                Kind::MessageCoinPredicate(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                Kind::MessageDataSigned(__self_0),
                                Kind::MessageDataSigned(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                Kind::MessageDataPredicate(__self_0),
                                Kind::MessageDataPredicate(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            _ => unsafe { ::core::intrinsics::unreachable() }
                        }
                }
            }
            impl Kind {
                /// Encodes the message to a buffer.
                pub fn encode<B>(&self, buf: &mut B)
                where
                    B: ::prost::bytes::BufMut,
                {
                    match *self {
                        Kind::CoinSigned(ref value) => {
                            ::prost::encoding::message::encode(1u32, &*value, buf);
                        }
                        Kind::CoinPredicate(ref value) => {
                            ::prost::encoding::message::encode(2u32, &*value, buf);
                        }
                        Kind::Contract(ref value) => {
                            ::prost::encoding::message::encode(3u32, &*value, buf);
                        }
                        Kind::MessageCoinSigned(ref value) => {
                            ::prost::encoding::message::encode(4u32, &*value, buf);
                        }
                        Kind::MessageCoinPredicate(ref value) => {
                            ::prost::encoding::message::encode(5u32, &*value, buf);
                        }
                        Kind::MessageDataSigned(ref value) => {
                            ::prost::encoding::message::encode(6u32, &*value, buf);
                        }
                        Kind::MessageDataPredicate(ref value) => {
                            ::prost::encoding::message::encode(7u32, &*value, buf);
                        }
                    }
                }
                /// Decodes an instance of the message from a buffer, and merges it into self.
                pub fn merge<B>(
                    field: &mut ::core::option::Option<Kind>,
                    tag: u32,
                    wire_type: ::prost::encoding::WireType,
                    buf: &mut B,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError>
                where
                    B: ::prost::bytes::Buf,
                {
                    match tag {
                        1u32 => {
                            match field {
                                ::core::option::Option::Some(
                                    Kind::CoinSigned(ref mut value),
                                ) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::CoinSigned(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        2u32 => {
                            match field {
                                ::core::option::Option::Some(
                                    Kind::CoinPredicate(ref mut value),
                                ) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::CoinPredicate(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        3u32 => {
                            match field {
                                ::core::option::Option::Some(
                                    Kind::Contract(ref mut value),
                                ) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::Contract(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        4u32 => {
                            match field {
                                ::core::option::Option::Some(
                                    Kind::MessageCoinSigned(ref mut value),
                                ) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::MessageCoinSigned(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        5u32 => {
                            match field {
                                ::core::option::Option::Some(
                                    Kind::MessageCoinPredicate(ref mut value),
                                ) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::MessageCoinPredicate(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        6u32 => {
                            match field {
                                ::core::option::Option::Some(
                                    Kind::MessageDataSigned(ref mut value),
                                ) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::MessageDataSigned(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        7u32 => {
                            match field {
                                ::core::option::Option::Some(
                                    Kind::MessageDataPredicate(ref mut value),
                                ) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::MessageDataPredicate(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["internal error: entered unreachable code: "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(
                                            &::core::fmt::Arguments::new_v1(
                                                &["invalid Kind tag: "],
                                                &[::core::fmt::ArgumentV1::new_display(&tag)],
                                            ),
                                        ),
                                    ],
                                ),
                            )
                        }
                    }
                }
                /// Returns the encoded length of the message without a length delimiter.
                #[inline]
                pub fn encoded_len(&self) -> usize {
                    match *self {
                        Kind::CoinSigned(ref value) => {
                            ::prost::encoding::message::encoded_len(1u32, &*value)
                        }
                        Kind::CoinPredicate(ref value) => {
                            ::prost::encoding::message::encoded_len(2u32, &*value)
                        }
                        Kind::Contract(ref value) => {
                            ::prost::encoding::message::encoded_len(3u32, &*value)
                        }
                        Kind::MessageCoinSigned(ref value) => {
                            ::prost::encoding::message::encoded_len(4u32, &*value)
                        }
                        Kind::MessageCoinPredicate(ref value) => {
                            ::prost::encoding::message::encoded_len(5u32, &*value)
                        }
                        Kind::MessageDataSigned(ref value) => {
                            ::prost::encoding::message::encoded_len(6u32, &*value)
                        }
                        Kind::MessageDataPredicate(ref value) => {
                            ::prost::encoding::message::encoded_len(7u32, &*value)
                        }
                    }
                }
            }
            impl ::core::fmt::Debug for Kind {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Kind::CoinSigned(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("CoinSigned").field(&wrapper).finish()
                        }
                        Kind::CoinPredicate(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("CoinPredicate").field(&wrapper).finish()
                        }
                        Kind::Contract(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("Contract").field(&wrapper).finish()
                        }
                        Kind::MessageCoinSigned(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("MessageCoinSigned").field(&wrapper).finish()
                        }
                        Kind::MessageCoinPredicate(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("MessageCoinPredicate")
                                .field(&wrapper)
                                .finish()
                        }
                        Kind::MessageDataSigned(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("MessageDataSigned").field(&wrapper).finish()
                        }
                        Kind::MessageDataPredicate(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("MessageDataPredicate")
                                .field(&wrapper)
                                .finish()
                        }
                    }
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct Coin {
            #[prost(message, optional, tag = "1")]
            pub utxo_id: ::core::option::Option<UtxoId>,
            #[prost(bytes = "vec", tag = "2")]
            pub owner: ::prost::alloc::vec::Vec<u8>,
            #[prost(uint64, tag = "3")]
            pub amount: u64,
            #[prost(bytes = "vec", tag = "4")]
            pub asset_id: ::prost::alloc::vec::Vec<u8>,
            #[prost(message, optional, tag = "5")]
            pub tx_pointer: ::core::option::Option<TxPointer>,
            #[prost(uint32, tag = "6")]
            pub witness_index: u32,
            #[prost(uint32, tag = "7")]
            pub maturity: u32,
            #[prost(uint64, tag = "8")]
            pub predicate_gas_used: u64,
            #[prost(bytes = "vec", tag = "9")]
            pub predicate: ::prost::alloc::vec::Vec<u8>,
            #[prost(bytes = "vec", tag = "10")]
            pub predicate_data: ::prost::alloc::vec::Vec<u8>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Coin {
            #[inline]
            fn clone(&self) -> Coin {
                Coin {
                    utxo_id: ::core::clone::Clone::clone(&self.utxo_id),
                    owner: ::core::clone::Clone::clone(&self.owner),
                    amount: ::core::clone::Clone::clone(&self.amount),
                    asset_id: ::core::clone::Clone::clone(&self.asset_id),
                    tx_pointer: ::core::clone::Clone::clone(&self.tx_pointer),
                    witness_index: ::core::clone::Clone::clone(&self.witness_index),
                    maturity: ::core::clone::Clone::clone(&self.maturity),
                    predicate_gas_used: ::core::clone::Clone::clone(
                        &self.predicate_gas_used,
                    ),
                    predicate: ::core::clone::Clone::clone(&self.predicate),
                    predicate_data: ::core::clone::Clone::clone(&self.predicate_data),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Coin {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Coin {
            #[inline]
            fn eq(&self, other: &Coin) -> bool {
                self.utxo_id == other.utxo_id && self.owner == other.owner
                    && self.amount == other.amount && self.asset_id == other.asset_id
                    && self.tx_pointer == other.tx_pointer
                    && self.witness_index == other.witness_index
                    && self.maturity == other.maturity
                    && self.predicate_gas_used == other.predicate_gas_used
                    && self.predicate == other.predicate
                    && self.predicate_data == other.predicate_data
            }
        }
        impl ::prost::Message for Coin {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if let Some(ref msg) = self.utxo_id {
                    ::prost::encoding::message::encode(1u32, msg, buf);
                }
                if self.owner != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(2u32, &self.owner, buf);
                }
                if self.amount != 0u64 {
                    ::prost::encoding::uint64::encode(3u32, &self.amount, buf);
                }
                if self.asset_id != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(4u32, &self.asset_id, buf);
                }
                if let Some(ref msg) = self.tx_pointer {
                    ::prost::encoding::message::encode(5u32, msg, buf);
                }
                if self.witness_index != 0u32 {
                    ::prost::encoding::uint32::encode(6u32, &self.witness_index, buf);
                }
                if self.maturity != 0u32 {
                    ::prost::encoding::uint32::encode(7u32, &self.maturity, buf);
                }
                if self.predicate_gas_used != 0u64 {
                    ::prost::encoding::uint64::encode(
                        8u32,
                        &self.predicate_gas_used,
                        buf,
                    );
                }
                if self.predicate != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(9u32, &self.predicate, buf);
                }
                if self.predicate_data != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(10u32, &self.predicate_data, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "Coin";
                match tag {
                    1u32 => {
                        let mut value = &mut self.utxo_id;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "utxo_id");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.owner;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "owner");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.amount;
                        ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "amount");
                                error
                            })
                    }
                    4u32 => {
                        let mut value = &mut self.asset_id;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "asset_id");
                                error
                            })
                    }
                    5u32 => {
                        let mut value = &mut self.tx_pointer;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "tx_pointer");
                                error
                            })
                    }
                    6u32 => {
                        let mut value = &mut self.witness_index;
                        ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "witness_index");
                                error
                            })
                    }
                    7u32 => {
                        let mut value = &mut self.maturity;
                        ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "maturity");
                                error
                            })
                    }
                    8u32 => {
                        let mut value = &mut self.predicate_gas_used;
                        ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "predicate_gas_used");
                                error
                            })
                    }
                    9u32 => {
                        let mut value = &mut self.predicate;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "predicate");
                                error
                            })
                    }
                    10u32 => {
                        let mut value = &mut self.predicate_data;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "predicate_data");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + self
                        .utxo_id
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                        )
                    + if self.owner != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(2u32, &self.owner)
                    } else {
                        0
                    }
                    + if self.amount != 0u64 {
                        ::prost::encoding::uint64::encoded_len(3u32, &self.amount)
                    } else {
                        0
                    }
                    + if self.asset_id != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(4u32, &self.asset_id)
                    } else {
                        0
                    }
                    + self
                        .tx_pointer
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(5u32, msg),
                        )
                    + if self.witness_index != 0u32 {
                        ::prost::encoding::uint32::encoded_len(6u32, &self.witness_index)
                    } else {
                        0
                    }
                    + if self.maturity != 0u32 {
                        ::prost::encoding::uint32::encoded_len(7u32, &self.maturity)
                    } else {
                        0
                    }
                    + if self.predicate_gas_used != 0u64 {
                        ::prost::encoding::uint64::encoded_len(
                            8u32,
                            &self.predicate_gas_used,
                        )
                    } else {
                        0
                    }
                    + if self.predicate != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(9u32, &self.predicate)
                    } else {
                        0
                    }
                    + if self.predicate_data != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(
                            10u32,
                            &self.predicate_data,
                        )
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.utxo_id = ::core::option::Option::None;
                self.owner.clear();
                self.amount = 0u64;
                self.asset_id.clear();
                self.tx_pointer = ::core::option::Option::None;
                self.witness_index = 0u32;
                self.maturity = 0u32;
                self.predicate_gas_used = 0u64;
                self.predicate.clear();
                self.predicate_data.clear();
            }
        }
        impl ::core::default::Default for Coin {
            fn default() -> Self {
                Coin {
                    utxo_id: ::core::default::Default::default(),
                    owner: ::core::default::Default::default(),
                    amount: 0u64,
                    asset_id: ::core::default::Default::default(),
                    tx_pointer: ::core::default::Default::default(),
                    witness_index: 0u32,
                    maturity: 0u32,
                    predicate_gas_used: 0u64,
                    predicate: ::core::default::Default::default(),
                    predicate_data: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for Coin {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Coin");
                let builder = {
                    let wrapper = &self.utxo_id;
                    builder.field("utxo_id", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.owner)
                    };
                    builder.field("owner", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.amount)
                    };
                    builder.field("amount", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.asset_id)
                    };
                    builder.field("asset_id", &wrapper)
                };
                let builder = {
                    let wrapper = &self.tx_pointer;
                    builder.field("tx_pointer", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.witness_index)
                    };
                    builder.field("witness_index", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.maturity)
                    };
                    builder.field("maturity", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.predicate_gas_used)
                    };
                    builder.field("predicate_gas_used", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.predicate)
                    };
                    builder.field("predicate", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.predicate_data)
                    };
                    builder.field("predicate_data", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        mod __coin__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscCoin> for Coin {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscCoin, graph::runtime::HostExportError> {
                    Ok(AscCoin {
                        utxo_id: graph::runtime::asc_new_or_null(
                            heap,
                            &self.utxo_id,
                            gas,
                        )?,
                        owner: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.owner),
                            gas,
                        )?,
                        amount: self.amount,
                        asset_id: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.asset_id),
                            gas,
                        )?,
                        tx_pointer: graph::runtime::asc_new_or_null(
                            heap,
                            &self.tx_pointer,
                            gas,
                        )?,
                        witness_index: self.witness_index,
                        maturity: self.maturity,
                        predicate_gas_used: self.predicate_gas_used,
                        predicate: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.predicate),
                            gas,
                        )?,
                        predicate_data: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(
                                &self.predicate_data,
                            ),
                            gas,
                        )?,
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscCoin {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelCoin;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscCoin {
            pub utxo_id: graph::runtime::AscPtr<crate::protobuf::AscUtxoId>,
            pub owner: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub amount: u64,
            pub asset_id: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub tx_pointer: graph::runtime::AscPtr<crate::protobuf::AscTxPointer>,
            pub witness_index: u32,
            pub maturity: u32,
            pub predicate_gas_used: u64,
            pub predicate: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub predicate_data: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscCoin {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "utxo_id",
                    "owner",
                    "amount",
                    "asset_id",
                    "tx_pointer",
                    "witness_index",
                    "maturity",
                    "predicate_gas_used",
                    "predicate",
                    "predicate_data",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.utxo_id,
                    &&self.owner,
                    &&self.amount,
                    &&self.asset_id,
                    &&self.tx_pointer,
                    &&self.witness_index,
                    &&self.maturity,
                    &&self.predicate_gas_used,
                    &&self.predicate,
                    &&self.predicate_data,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "AscCoin",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscCoin {
            #[inline]
            fn default() -> AscCoin {
                AscCoin {
                    utxo_id: ::core::default::Default::default(),
                    owner: ::core::default::Default::default(),
                    amount: ::core::default::Default::default(),
                    asset_id: ::core::default::Default::default(),
                    tx_pointer: ::core::default::Default::default(),
                    witness_index: ::core::default::Default::default(),
                    maturity: ::core::default::Default::default(),
                    predicate_gas_used: ::core::default::Default::default(),
                    predicate: ::core::default::Default::default(),
                    predicate_data: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscCoin {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscUtxoId>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.utxo_id.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.owner.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.amount.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.asset_id.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscTxPointer>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.tx_pointer.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.witness_index.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.maturity.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.predicate_gas_used.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.predicate.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.predicate_data.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscCoin")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscUtxoId>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscUtxoId>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let utxo_id = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let owner = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u64>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let amount = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let asset_id = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscTxPointer>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscTxPointer>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let tx_pointer = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u32>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let witness_index = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u32>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let maturity = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u64>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let predicate_gas_used = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let predicate = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let predicate_data = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self {
                    utxo_id,
                    owner,
                    amount,
                    asset_id,
                    tx_pointer,
                    witness_index,
                    maturity,
                    predicate_gas_used,
                    predicate,
                    predicate_data,
                })
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct Message {
            #[prost(bytes = "vec", tag = "1")]
            pub sender: ::prost::alloc::vec::Vec<u8>,
            #[prost(bytes = "vec", tag = "2")]
            pub recipient: ::prost::alloc::vec::Vec<u8>,
            #[prost(uint64, tag = "3")]
            pub amount: u64,
            #[prost(bytes = "vec", tag = "4")]
            pub nonce: ::prost::alloc::vec::Vec<u8>,
            #[prost(uint32, tag = "5")]
            pub witness_index: u32,
            #[prost(uint64, tag = "6")]
            pub predicate_gas_used: u64,
            #[prost(bytes = "vec", tag = "7")]
            pub data: ::prost::alloc::vec::Vec<u8>,
            #[prost(bytes = "vec", tag = "8")]
            pub predicate: ::prost::alloc::vec::Vec<u8>,
            #[prost(bytes = "vec", tag = "9")]
            pub predicate_data: ::prost::alloc::vec::Vec<u8>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Message {
            #[inline]
            fn clone(&self) -> Message {
                Message {
                    sender: ::core::clone::Clone::clone(&self.sender),
                    recipient: ::core::clone::Clone::clone(&self.recipient),
                    amount: ::core::clone::Clone::clone(&self.amount),
                    nonce: ::core::clone::Clone::clone(&self.nonce),
                    witness_index: ::core::clone::Clone::clone(&self.witness_index),
                    predicate_gas_used: ::core::clone::Clone::clone(
                        &self.predicate_gas_used,
                    ),
                    data: ::core::clone::Clone::clone(&self.data),
                    predicate: ::core::clone::Clone::clone(&self.predicate),
                    predicate_data: ::core::clone::Clone::clone(&self.predicate_data),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Message {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Message {
            #[inline]
            fn eq(&self, other: &Message) -> bool {
                self.sender == other.sender && self.recipient == other.recipient
                    && self.amount == other.amount && self.nonce == other.nonce
                    && self.witness_index == other.witness_index
                    && self.predicate_gas_used == other.predicate_gas_used
                    && self.data == other.data && self.predicate == other.predicate
                    && self.predicate_data == other.predicate_data
            }
        }
        impl ::prost::Message for Message {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if self.sender != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(1u32, &self.sender, buf);
                }
                if self.recipient != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(2u32, &self.recipient, buf);
                }
                if self.amount != 0u64 {
                    ::prost::encoding::uint64::encode(3u32, &self.amount, buf);
                }
                if self.nonce != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(4u32, &self.nonce, buf);
                }
                if self.witness_index != 0u32 {
                    ::prost::encoding::uint32::encode(5u32, &self.witness_index, buf);
                }
                if self.predicate_gas_used != 0u64 {
                    ::prost::encoding::uint64::encode(
                        6u32,
                        &self.predicate_gas_used,
                        buf,
                    );
                }
                if self.data != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(7u32, &self.data, buf);
                }
                if self.predicate != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(8u32, &self.predicate, buf);
                }
                if self.predicate_data != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(9u32, &self.predicate_data, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "Message";
                match tag {
                    1u32 => {
                        let mut value = &mut self.sender;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "sender");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.recipient;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "recipient");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.amount;
                        ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "amount");
                                error
                            })
                    }
                    4u32 => {
                        let mut value = &mut self.nonce;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "nonce");
                                error
                            })
                    }
                    5u32 => {
                        let mut value = &mut self.witness_index;
                        ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "witness_index");
                                error
                            })
                    }
                    6u32 => {
                        let mut value = &mut self.predicate_gas_used;
                        ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "predicate_gas_used");
                                error
                            })
                    }
                    7u32 => {
                        let mut value = &mut self.data;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "data");
                                error
                            })
                    }
                    8u32 => {
                        let mut value = &mut self.predicate;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "predicate");
                                error
                            })
                    }
                    9u32 => {
                        let mut value = &mut self.predicate_data;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "predicate_data");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.sender != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(1u32, &self.sender)
                    } else {
                        0
                    }
                    + if self.recipient != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(2u32, &self.recipient)
                    } else {
                        0
                    }
                    + if self.amount != 0u64 {
                        ::prost::encoding::uint64::encoded_len(3u32, &self.amount)
                    } else {
                        0
                    }
                    + if self.nonce != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(4u32, &self.nonce)
                    } else {
                        0
                    }
                    + if self.witness_index != 0u32 {
                        ::prost::encoding::uint32::encoded_len(5u32, &self.witness_index)
                    } else {
                        0
                    }
                    + if self.predicate_gas_used != 0u64 {
                        ::prost::encoding::uint64::encoded_len(
                            6u32,
                            &self.predicate_gas_used,
                        )
                    } else {
                        0
                    }
                    + if self.data != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(7u32, &self.data)
                    } else {
                        0
                    }
                    + if self.predicate != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(8u32, &self.predicate)
                    } else {
                        0
                    }
                    + if self.predicate_data != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(9u32, &self.predicate_data)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.sender.clear();
                self.recipient.clear();
                self.amount = 0u64;
                self.nonce.clear();
                self.witness_index = 0u32;
                self.predicate_gas_used = 0u64;
                self.data.clear();
                self.predicate.clear();
                self.predicate_data.clear();
            }
        }
        impl ::core::default::Default for Message {
            fn default() -> Self {
                Message {
                    sender: ::core::default::Default::default(),
                    recipient: ::core::default::Default::default(),
                    amount: 0u64,
                    nonce: ::core::default::Default::default(),
                    witness_index: 0u32,
                    predicate_gas_used: 0u64,
                    data: ::core::default::Default::default(),
                    predicate: ::core::default::Default::default(),
                    predicate_data: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for Message {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Message");
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.sender)
                    };
                    builder.field("sender", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.recipient)
                    };
                    builder.field("recipient", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.amount)
                    };
                    builder.field("amount", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.nonce)
                    };
                    builder.field("nonce", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.witness_index)
                    };
                    builder.field("witness_index", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.predicate_gas_used)
                    };
                    builder.field("predicate_gas_used", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.data)
                    };
                    builder.field("data", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.predicate)
                    };
                    builder.field("predicate", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.predicate_data)
                    };
                    builder.field("predicate_data", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        mod __message__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscMessage> for Message {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscMessage, graph::runtime::HostExportError> {
                    Ok(AscMessage {
                        sender: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.sender),
                            gas,
                        )?,
                        recipient: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.recipient),
                            gas,
                        )?,
                        amount: self.amount,
                        nonce: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.nonce),
                            gas,
                        )?,
                        witness_index: self.witness_index,
                        predicate_gas_used: self.predicate_gas_used,
                        data: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.data),
                            gas,
                        )?,
                        predicate: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.predicate),
                            gas,
                        )?,
                        predicate_data: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(
                                &self.predicate_data,
                            ),
                            gas,
                        )?,
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscMessage {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelMessage;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscMessage {
            pub sender: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub recipient: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub amount: u64,
            pub nonce: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub witness_index: u32,
            pub predicate_gas_used: u64,
            pub data: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub predicate: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub predicate_data: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscMessage {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "sender",
                    "recipient",
                    "amount",
                    "nonce",
                    "witness_index",
                    "predicate_gas_used",
                    "data",
                    "predicate",
                    "predicate_data",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.sender,
                    &&self.recipient,
                    &&self.amount,
                    &&self.nonce,
                    &&self.witness_index,
                    &&self.predicate_gas_used,
                    &&self.data,
                    &&self.predicate,
                    &&self.predicate_data,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "AscMessage",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscMessage {
            #[inline]
            fn default() -> AscMessage {
                AscMessage {
                    sender: ::core::default::Default::default(),
                    recipient: ::core::default::Default::default(),
                    amount: ::core::default::Default::default(),
                    nonce: ::core::default::Default::default(),
                    witness_index: ::core::default::Default::default(),
                    predicate_gas_used: ::core::default::Default::default(),
                    data: ::core::default::Default::default(),
                    predicate: ::core::default::Default::default(),
                    predicate_data: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscMessage {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.sender.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.recipient.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.amount.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.nonce.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.witness_index.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.predicate_gas_used.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.data.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.predicate.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.predicate_data.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscMessage")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let sender = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let recipient = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u64>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let amount = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let nonce = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u32>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let witness_index = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u64>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let predicate_gas_used = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let data = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let predicate = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let predicate_data = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self {
                    sender,
                    recipient,
                    amount,
                    nonce,
                    witness_index,
                    predicate_gas_used,
                    data,
                    predicate,
                    predicate_data,
                })
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct Output {
            #[prost(oneof = "output::Kind", tags = "1, 2, 3, 4, 5")]
            pub kind: ::core::option::Option<output::Kind>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Output {
            #[inline]
            fn clone(&self) -> Output {
                Output {
                    kind: ::core::clone::Clone::clone(&self.kind),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Output {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Output {
            #[inline]
            fn eq(&self, other: &Output) -> bool {
                self.kind == other.kind
            }
        }
        impl ::prost::Message for Output {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if let Some(ref oneof) = self.kind {
                    oneof.encode(buf)
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "Output";
                match tag {
                    1u32 | 2u32 | 3u32 | 4u32 | 5u32 => {
                        let mut value = &mut self.kind;
                        output::Kind::merge(value, tag, wire_type, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "kind");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + self.kind.as_ref().map_or(0, output::Kind::encoded_len)
            }
            fn clear(&mut self) {
                self.kind = ::core::option::Option::None;
            }
        }
        impl ::core::default::Default for Output {
            fn default() -> Self {
                Output {
                    kind: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for Output {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Output");
                let builder = {
                    let wrapper = &self.kind;
                    builder.field("kind", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        pub struct AscOutputArray(
            pub graph_runtime_wasm::asc_abi::class::Array<
                graph::runtime::AscPtr<AscOutput>,
            >,
        );
        impl graph::runtime::ToAscObj<AscOutputArray> for Vec<Output> {
            fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                &self,
                heap: &mut H,
                gas: &graph::runtime::gas::GasCounter,
            ) -> Result<AscOutputArray, graph::runtime::HostExportError> {
                let content: Result<Vec<_>, _> = self
                    .iter()
                    .map(|x| graph::runtime::asc_new(heap, x, gas))
                    .collect();
                Ok(
                    AscOutputArray(
                        graph_runtime_wasm::asc_abi::class::Array::new(
                            &content?,
                            heap,
                            gas,
                        )?,
                    ),
                )
            }
        }
        impl graph::runtime::AscType for AscOutputArray {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                self.0.to_asc_bytes()
            }
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                Ok(
                    Self(
                        graph_runtime_wasm::asc_abi::class::Array::from_asc_bytes(
                            asc_obj,
                            api_version,
                        )?,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscOutputArray {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelOutputArray;
        }
        #[automatically_derived]
        mod __output__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscOutput> for Output {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscOutput, graph::runtime::HostExportError> {
                    let kind = self
                        .kind
                        .as_ref()
                        .ok_or_else(|| graph::runtime::HostExportError::from(
                            graph::runtime::DeterministicHostError::from(
                                ::anyhow::Error::msg({
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", " missing "],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(&"Output"),
                                                ::core::fmt::ArgumentV1::new_display(&"kind"),
                                            ],
                                        ),
                                    );
                                    res
                                }),
                            ),
                        ))?;
                    Ok(AscOutput {
                        coin: if let output::Kind::Coin(v) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        contract: if let output::Kind::Contract(v) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        change: if let output::Kind::Change(v) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        variable: if let output::Kind::Variable(v) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        contract_created: if let output::Kind::ContractCreated(v) = kind {
                            graph::runtime::asc_new(heap, v, gas)?
                        } else {
                            graph::runtime::AscPtr::null()
                        },
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscOutput {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelOutput;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscOutput {
            pub coin: graph::runtime::AscPtr<AscOutputCoin>,
            pub contract: graph::runtime::AscPtr<AscOutputContract>,
            pub change: graph::runtime::AscPtr<AscOutputCoin>,
            pub variable: graph::runtime::AscPtr<AscOutputCoin>,
            pub contract_created: graph::runtime::AscPtr<AscOutputContractCreated>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscOutput {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "AscOutput",
                    "coin",
                    &&self.coin,
                    "contract",
                    &&self.contract,
                    "change",
                    &&self.change,
                    "variable",
                    &&self.variable,
                    "contract_created",
                    &&self.contract_created,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscOutput {
            #[inline]
            fn default() -> AscOutput {
                AscOutput {
                    coin: ::core::default::Default::default(),
                    contract: ::core::default::Default::default(),
                    change: ::core::default::Default::default(),
                    variable: ::core::default::Default::default(),
                    contract_created: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscOutput {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscOutputCoin>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.coin.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscOutputContract>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.contract.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscOutputCoin>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.change.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscOutputCoin>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.variable.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscOutputContractCreated>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.contract_created.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscOutput")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscOutputCoin>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<AscOutputCoin>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let coin = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscOutputContract>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<AscOutputContract>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let contract = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscOutputCoin>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<AscOutputCoin>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let change = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscOutputCoin>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<AscOutputCoin>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let variable = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<AscOutputContractCreated>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<AscOutputContractCreated>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let contract_created = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self {
                    coin,
                    contract,
                    change,
                    variable,
                    contract_created,
                })
            }
        }
        /// Nested message and enum types in `Output`.
        pub mod output {
            #[allow(clippy::derive_partial_eq_without_eq)]
            pub enum Kind {
                #[prost(message, tag = "1")]
                Coin(super::OutputCoin),
                #[prost(message, tag = "2")]
                Contract(super::OutputContract),
                #[prost(message, tag = "3")]
                Change(super::OutputCoin),
                #[prost(message, tag = "4")]
                Variable(super::OutputCoin),
                #[prost(message, tag = "5")]
                ContractCreated(super::OutputContractCreated),
            }
            #[automatically_derived]
            #[allow(clippy::derive_partial_eq_without_eq)]
            impl ::core::clone::Clone for Kind {
                #[inline]
                fn clone(&self) -> Kind {
                    match self {
                        Kind::Coin(__self_0) => {
                            Kind::Coin(::core::clone::Clone::clone(__self_0))
                        }
                        Kind::Contract(__self_0) => {
                            Kind::Contract(::core::clone::Clone::clone(__self_0))
                        }
                        Kind::Change(__self_0) => {
                            Kind::Change(::core::clone::Clone::clone(__self_0))
                        }
                        Kind::Variable(__self_0) => {
                            Kind::Variable(::core::clone::Clone::clone(__self_0))
                        }
                        Kind::ContractCreated(__self_0) => {
                            Kind::ContractCreated(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Kind {}
            #[automatically_derived]
            #[allow(clippy::derive_partial_eq_without_eq)]
            impl ::core::cmp::PartialEq for Kind {
                #[inline]
                fn eq(&self, other: &Kind) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                        && match (self, other) {
                            (Kind::Coin(__self_0), Kind::Coin(__arg1_0)) => {
                                *__self_0 == *__arg1_0
                            }
                            (Kind::Contract(__self_0), Kind::Contract(__arg1_0)) => {
                                *__self_0 == *__arg1_0
                            }
                            (Kind::Change(__self_0), Kind::Change(__arg1_0)) => {
                                *__self_0 == *__arg1_0
                            }
                            (Kind::Variable(__self_0), Kind::Variable(__arg1_0)) => {
                                *__self_0 == *__arg1_0
                            }
                            (
                                Kind::ContractCreated(__self_0),
                                Kind::ContractCreated(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            _ => unsafe { ::core::intrinsics::unreachable() }
                        }
                }
            }
            impl Kind {
                /// Encodes the message to a buffer.
                pub fn encode<B>(&self, buf: &mut B)
                where
                    B: ::prost::bytes::BufMut,
                {
                    match *self {
                        Kind::Coin(ref value) => {
                            ::prost::encoding::message::encode(1u32, &*value, buf);
                        }
                        Kind::Contract(ref value) => {
                            ::prost::encoding::message::encode(2u32, &*value, buf);
                        }
                        Kind::Change(ref value) => {
                            ::prost::encoding::message::encode(3u32, &*value, buf);
                        }
                        Kind::Variable(ref value) => {
                            ::prost::encoding::message::encode(4u32, &*value, buf);
                        }
                        Kind::ContractCreated(ref value) => {
                            ::prost::encoding::message::encode(5u32, &*value, buf);
                        }
                    }
                }
                /// Decodes an instance of the message from a buffer, and merges it into self.
                pub fn merge<B>(
                    field: &mut ::core::option::Option<Kind>,
                    tag: u32,
                    wire_type: ::prost::encoding::WireType,
                    buf: &mut B,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError>
                where
                    B: ::prost::bytes::Buf,
                {
                    match tag {
                        1u32 => {
                            match field {
                                ::core::option::Option::Some(Kind::Coin(ref mut value)) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::Coin(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        2u32 => {
                            match field {
                                ::core::option::Option::Some(
                                    Kind::Contract(ref mut value),
                                ) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::Contract(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        3u32 => {
                            match field {
                                ::core::option::Option::Some(
                                    Kind::Change(ref mut value),
                                ) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::Change(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        4u32 => {
                            match field {
                                ::core::option::Option::Some(
                                    Kind::Variable(ref mut value),
                                ) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::Variable(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        5u32 => {
                            match field {
                                ::core::option::Option::Some(
                                    Kind::ContractCreated(ref mut value),
                                ) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Kind::ContractCreated(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["internal error: entered unreachable code: "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(
                                            &::core::fmt::Arguments::new_v1(
                                                &["invalid Kind tag: "],
                                                &[::core::fmt::ArgumentV1::new_display(&tag)],
                                            ),
                                        ),
                                    ],
                                ),
                            )
                        }
                    }
                }
                /// Returns the encoded length of the message without a length delimiter.
                #[inline]
                pub fn encoded_len(&self) -> usize {
                    match *self {
                        Kind::Coin(ref value) => {
                            ::prost::encoding::message::encoded_len(1u32, &*value)
                        }
                        Kind::Contract(ref value) => {
                            ::prost::encoding::message::encoded_len(2u32, &*value)
                        }
                        Kind::Change(ref value) => {
                            ::prost::encoding::message::encoded_len(3u32, &*value)
                        }
                        Kind::Variable(ref value) => {
                            ::prost::encoding::message::encoded_len(4u32, &*value)
                        }
                        Kind::ContractCreated(ref value) => {
                            ::prost::encoding::message::encoded_len(5u32, &*value)
                        }
                    }
                }
            }
            impl ::core::fmt::Debug for Kind {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Kind::Coin(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("Coin").field(&wrapper).finish()
                        }
                        Kind::Contract(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("Contract").field(&wrapper).finish()
                        }
                        Kind::Change(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("Change").field(&wrapper).finish()
                        }
                        Kind::Variable(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("Variable").field(&wrapper).finish()
                        }
                        Kind::ContractCreated(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("ContractCreated").field(&wrapper).finish()
                        }
                    }
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct OutputCoin {
            #[prost(bytes = "vec", tag = "1")]
            pub to: ::prost::alloc::vec::Vec<u8>,
            #[prost(uint64, tag = "2")]
            pub amount: u64,
            #[prost(bytes = "vec", tag = "3")]
            pub asset_id: ::prost::alloc::vec::Vec<u8>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for OutputCoin {
            #[inline]
            fn clone(&self) -> OutputCoin {
                OutputCoin {
                    to: ::core::clone::Clone::clone(&self.to),
                    amount: ::core::clone::Clone::clone(&self.amount),
                    asset_id: ::core::clone::Clone::clone(&self.asset_id),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for OutputCoin {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for OutputCoin {
            #[inline]
            fn eq(&self, other: &OutputCoin) -> bool {
                self.to == other.to && self.amount == other.amount
                    && self.asset_id == other.asset_id
            }
        }
        impl ::prost::Message for OutputCoin {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if self.to != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(1u32, &self.to, buf);
                }
                if self.amount != 0u64 {
                    ::prost::encoding::uint64::encode(2u32, &self.amount, buf);
                }
                if self.asset_id != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(3u32, &self.asset_id, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "OutputCoin";
                match tag {
                    1u32 => {
                        let mut value = &mut self.to;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "to");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.amount;
                        ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "amount");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.asset_id;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "asset_id");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.to != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(1u32, &self.to)
                    } else {
                        0
                    }
                    + if self.amount != 0u64 {
                        ::prost::encoding::uint64::encoded_len(2u32, &self.amount)
                    } else {
                        0
                    }
                    + if self.asset_id != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(3u32, &self.asset_id)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.to.clear();
                self.amount = 0u64;
                self.asset_id.clear();
            }
        }
        impl ::core::default::Default for OutputCoin {
            fn default() -> Self {
                OutputCoin {
                    to: ::core::default::Default::default(),
                    amount: 0u64,
                    asset_id: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for OutputCoin {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("OutputCoin");
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.to)
                    };
                    builder.field("to", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.amount)
                    };
                    builder.field("amount", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.asset_id)
                    };
                    builder.field("asset_id", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        mod __outputcoin__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscOutputCoin> for OutputCoin {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscOutputCoin, graph::runtime::HostExportError> {
                    Ok(AscOutputCoin {
                        to: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.to),
                            gas,
                        )?,
                        amount: self.amount,
                        asset_id: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.asset_id),
                            gas,
                        )?,
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscOutputCoin {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelOutputCoin;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscOutputCoin {
            pub to: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub amount: u64,
            pub asset_id: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscOutputCoin {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "AscOutputCoin",
                    "to",
                    &&self.to,
                    "amount",
                    &&self.amount,
                    "asset_id",
                    &&self.asset_id,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscOutputCoin {
            #[inline]
            fn default() -> AscOutputCoin {
                AscOutputCoin {
                    to: ::core::default::Default::default(),
                    amount: ::core::default::Default::default(),
                    asset_id: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscOutputCoin {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.to.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.amount.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.asset_id.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscOutputCoin")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let to = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u64>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u64>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let amount = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let asset_id = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self { to, amount, asset_id })
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct OutputContractCreated {
            #[prost(bytes = "vec", tag = "1")]
            pub contract_id: ::prost::alloc::vec::Vec<u8>,
            #[prost(bytes = "vec", tag = "2")]
            pub state_root: ::prost::alloc::vec::Vec<u8>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for OutputContractCreated {
            #[inline]
            fn clone(&self) -> OutputContractCreated {
                OutputContractCreated {
                    contract_id: ::core::clone::Clone::clone(&self.contract_id),
                    state_root: ::core::clone::Clone::clone(&self.state_root),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for OutputContractCreated {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for OutputContractCreated {
            #[inline]
            fn eq(&self, other: &OutputContractCreated) -> bool {
                self.contract_id == other.contract_id
                    && self.state_root == other.state_root
            }
        }
        impl ::prost::Message for OutputContractCreated {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if self.contract_id != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(1u32, &self.contract_id, buf);
                }
                if self.state_root != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(2u32, &self.state_root, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "OutputContractCreated";
                match tag {
                    1u32 => {
                        let mut value = &mut self.contract_id;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "contract_id");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.state_root;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "state_root");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.contract_id != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(1u32, &self.contract_id)
                    } else {
                        0
                    }
                    + if self.state_root != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(2u32, &self.state_root)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.contract_id.clear();
                self.state_root.clear();
            }
        }
        impl ::core::default::Default for OutputContractCreated {
            fn default() -> Self {
                OutputContractCreated {
                    contract_id: ::core::default::Default::default(),
                    state_root: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for OutputContractCreated {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("OutputContractCreated");
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.contract_id)
                    };
                    builder.field("contract_id", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.state_root)
                    };
                    builder.field("state_root", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        mod __outputcontractcreated__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscOutputContractCreated>
            for OutputContractCreated {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscOutputContractCreated, graph::runtime::HostExportError> {
                    Ok(AscOutputContractCreated {
                        contract_id: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(
                                &self.contract_id,
                            ),
                            gas,
                        )?,
                        state_root: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.state_root),
                            gas,
                        )?,
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscOutputContractCreated {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelOutputContractCreated;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscOutputContractCreated {
            pub contract_id: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub state_root: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscOutputContractCreated {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "AscOutputContractCreated",
                    "contract_id",
                    &&self.contract_id,
                    "state_root",
                    &&self.state_root,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscOutputContractCreated {
            #[inline]
            fn default() -> AscOutputContractCreated {
                AscOutputContractCreated {
                    contract_id: ::core::default::Default::default(),
                    state_root: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscOutputContractCreated {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.contract_id.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.state_root.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[
                                            ::core::fmt::ArgumentV1::new_display(
                                                &"AscOutputContractCreated",
                                            ),
                                        ],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let contract_id = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let state_root = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self { contract_id, state_root })
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct InputContract {
            #[prost(message, optional, tag = "1")]
            pub utxo_id: ::core::option::Option<UtxoId>,
            #[prost(bytes = "vec", tag = "2")]
            pub balance_root: ::prost::alloc::vec::Vec<u8>,
            #[prost(bytes = "vec", tag = "3")]
            pub state_root: ::prost::alloc::vec::Vec<u8>,
            #[prost(message, optional, tag = "4")]
            pub tx_pointer: ::core::option::Option<TxPointer>,
            #[prost(bytes = "vec", tag = "5")]
            pub contract_id: ::prost::alloc::vec::Vec<u8>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for InputContract {
            #[inline]
            fn clone(&self) -> InputContract {
                InputContract {
                    utxo_id: ::core::clone::Clone::clone(&self.utxo_id),
                    balance_root: ::core::clone::Clone::clone(&self.balance_root),
                    state_root: ::core::clone::Clone::clone(&self.state_root),
                    tx_pointer: ::core::clone::Clone::clone(&self.tx_pointer),
                    contract_id: ::core::clone::Clone::clone(&self.contract_id),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for InputContract {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for InputContract {
            #[inline]
            fn eq(&self, other: &InputContract) -> bool {
                self.utxo_id == other.utxo_id && self.balance_root == other.balance_root
                    && self.state_root == other.state_root
                    && self.tx_pointer == other.tx_pointer
                    && self.contract_id == other.contract_id
            }
        }
        impl ::prost::Message for InputContract {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if let Some(ref msg) = self.utxo_id {
                    ::prost::encoding::message::encode(1u32, msg, buf);
                }
                if self.balance_root != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(2u32, &self.balance_root, buf);
                }
                if self.state_root != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(3u32, &self.state_root, buf);
                }
                if let Some(ref msg) = self.tx_pointer {
                    ::prost::encoding::message::encode(4u32, msg, buf);
                }
                if self.contract_id != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(5u32, &self.contract_id, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "InputContract";
                match tag {
                    1u32 => {
                        let mut value = &mut self.utxo_id;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "utxo_id");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.balance_root;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "balance_root");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.state_root;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "state_root");
                                error
                            })
                    }
                    4u32 => {
                        let mut value = &mut self.tx_pointer;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "tx_pointer");
                                error
                            })
                    }
                    5u32 => {
                        let mut value = &mut self.contract_id;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "contract_id");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + self
                        .utxo_id
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                        )
                    + if self.balance_root != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(2u32, &self.balance_root)
                    } else {
                        0
                    }
                    + if self.state_root != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(3u32, &self.state_root)
                    } else {
                        0
                    }
                    + self
                        .tx_pointer
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(4u32, msg),
                        )
                    + if self.contract_id != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(5u32, &self.contract_id)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.utxo_id = ::core::option::Option::None;
                self.balance_root.clear();
                self.state_root.clear();
                self.tx_pointer = ::core::option::Option::None;
                self.contract_id.clear();
            }
        }
        impl ::core::default::Default for InputContract {
            fn default() -> Self {
                InputContract {
                    utxo_id: ::core::default::Default::default(),
                    balance_root: ::core::default::Default::default(),
                    state_root: ::core::default::Default::default(),
                    tx_pointer: ::core::default::Default::default(),
                    contract_id: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for InputContract {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("InputContract");
                let builder = {
                    let wrapper = &self.utxo_id;
                    builder.field("utxo_id", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.balance_root)
                    };
                    builder.field("balance_root", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.state_root)
                    };
                    builder.field("state_root", &wrapper)
                };
                let builder = {
                    let wrapper = &self.tx_pointer;
                    builder.field("tx_pointer", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.contract_id)
                    };
                    builder.field("contract_id", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        mod __inputcontract__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscInputContract> for InputContract {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscInputContract, graph::runtime::HostExportError> {
                    Ok(AscInputContract {
                        utxo_id: graph::runtime::asc_new_or_null(
                            heap,
                            &self.utxo_id,
                            gas,
                        )?,
                        balance_root: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(
                                &self.balance_root,
                            ),
                            gas,
                        )?,
                        state_root: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.state_root),
                            gas,
                        )?,
                        tx_pointer: graph::runtime::asc_new_or_null(
                            heap,
                            &self.tx_pointer,
                            gas,
                        )?,
                        contract_id: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(
                                &self.contract_id,
                            ),
                            gas,
                        )?,
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscInputContract {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelInputContract;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscInputContract {
            pub utxo_id: graph::runtime::AscPtr<crate::protobuf::AscUtxoId>,
            pub balance_root: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub state_root: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub tx_pointer: graph::runtime::AscPtr<crate::protobuf::AscTxPointer>,
            pub contract_id: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscInputContract {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "AscInputContract",
                    "utxo_id",
                    &&self.utxo_id,
                    "balance_root",
                    &&self.balance_root,
                    "state_root",
                    &&self.state_root,
                    "tx_pointer",
                    &&self.tx_pointer,
                    "contract_id",
                    &&self.contract_id,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscInputContract {
            #[inline]
            fn default() -> AscInputContract {
                AscInputContract {
                    utxo_id: ::core::default::Default::default(),
                    balance_root: ::core::default::Default::default(),
                    state_root: ::core::default::Default::default(),
                    tx_pointer: ::core::default::Default::default(),
                    contract_id: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscInputContract {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscUtxoId>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.utxo_id.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.balance_root.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.state_root.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscTxPointer>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.tx_pointer.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.contract_id.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscInputContract")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscUtxoId>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscUtxoId>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let utxo_id = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let balance_root = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let state_root = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscTxPointer>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::AscTxPointer>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let tx_pointer = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let contract_id = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self {
                    utxo_id,
                    balance_root,
                    state_root,
                    tx_pointer,
                    contract_id,
                })
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct OutputContract {
            #[prost(uint32, tag = "1")]
            pub input_index: u32,
            #[prost(bytes = "vec", tag = "2")]
            pub balance_root: ::prost::alloc::vec::Vec<u8>,
            #[prost(bytes = "vec", tag = "3")]
            pub state_root: ::prost::alloc::vec::Vec<u8>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for OutputContract {
            #[inline]
            fn clone(&self) -> OutputContract {
                OutputContract {
                    input_index: ::core::clone::Clone::clone(&self.input_index),
                    balance_root: ::core::clone::Clone::clone(&self.balance_root),
                    state_root: ::core::clone::Clone::clone(&self.state_root),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for OutputContract {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for OutputContract {
            #[inline]
            fn eq(&self, other: &OutputContract) -> bool {
                self.input_index == other.input_index
                    && self.balance_root == other.balance_root
                    && self.state_root == other.state_root
            }
        }
        impl ::prost::Message for OutputContract {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if self.input_index != 0u32 {
                    ::prost::encoding::uint32::encode(1u32, &self.input_index, buf);
                }
                if self.balance_root != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(2u32, &self.balance_root, buf);
                }
                if self.state_root != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(3u32, &self.state_root, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "OutputContract";
                match tag {
                    1u32 => {
                        let mut value = &mut self.input_index;
                        ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "input_index");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.balance_root;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "balance_root");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.state_root;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "state_root");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.input_index != 0u32 {
                        ::prost::encoding::uint32::encoded_len(1u32, &self.input_index)
                    } else {
                        0
                    }
                    + if self.balance_root != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(2u32, &self.balance_root)
                    } else {
                        0
                    }
                    + if self.state_root != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(3u32, &self.state_root)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.input_index = 0u32;
                self.balance_root.clear();
                self.state_root.clear();
            }
        }
        impl ::core::default::Default for OutputContract {
            fn default() -> Self {
                OutputContract {
                    input_index: 0u32,
                    balance_root: ::core::default::Default::default(),
                    state_root: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for OutputContract {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("OutputContract");
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.input_index)
                    };
                    builder.field("input_index", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.balance_root)
                    };
                    builder.field("balance_root", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.state_root)
                    };
                    builder.field("state_root", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        mod __outputcontract__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscOutputContract> for OutputContract {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscOutputContract, graph::runtime::HostExportError> {
                    Ok(AscOutputContract {
                        input_index: self.input_index,
                        balance_root: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(
                                &self.balance_root,
                            ),
                            gas,
                        )?,
                        state_root: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.state_root),
                            gas,
                        )?,
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscOutputContract {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelOutputContract;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscOutputContract {
            pub input_index: u32,
            pub balance_root: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub state_root: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscOutputContract {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "AscOutputContract",
                    "input_index",
                    &&self.input_index,
                    "balance_root",
                    &&self.balance_root,
                    "state_root",
                    &&self.state_root,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscOutputContract {
            #[inline]
            fn default() -> AscOutputContract {
                AscOutputContract {
                    input_index: ::core::default::Default::default(),
                    balance_root: ::core::default::Default::default(),
                    state_root: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscOutputContract {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.input_index.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.balance_root.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.state_root.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[
                                            ::core::fmt::ArgumentV1::new_display(&"AscOutputContract"),
                                        ],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u32>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let input_index = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let balance_root = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let state_root = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self {
                    input_index,
                    balance_root,
                    state_root,
                })
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct StorageSlot {
            #[prost(bytes = "vec", tag = "1")]
            pub key: ::prost::alloc::vec::Vec<u8>,
            #[prost(bytes = "vec", tag = "2")]
            pub value: ::prost::alloc::vec::Vec<u8>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for StorageSlot {
            #[inline]
            fn clone(&self) -> StorageSlot {
                StorageSlot {
                    key: ::core::clone::Clone::clone(&self.key),
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for StorageSlot {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for StorageSlot {
            #[inline]
            fn eq(&self, other: &StorageSlot) -> bool {
                self.key == other.key && self.value == other.value
            }
        }
        impl ::prost::Message for StorageSlot {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if self.key != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(1u32, &self.key, buf);
                }
                if self.value != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(2u32, &self.value, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "StorageSlot";
                match tag {
                    1u32 => {
                        let mut value = &mut self.key;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "key");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.value;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "value");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.key != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(1u32, &self.key)
                    } else {
                        0
                    }
                    + if self.value != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(2u32, &self.value)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.key.clear();
                self.value.clear();
            }
        }
        impl ::core::default::Default for StorageSlot {
            fn default() -> Self {
                StorageSlot {
                    key: ::core::default::Default::default(),
                    value: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for StorageSlot {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("StorageSlot");
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.key)
                    };
                    builder.field("key", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.value)
                    };
                    builder.field("value", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        pub struct AscStorageSlotArray(
            pub graph_runtime_wasm::asc_abi::class::Array<
                graph::runtime::AscPtr<AscStorageSlot>,
            >,
        );
        impl graph::runtime::ToAscObj<AscStorageSlotArray> for Vec<StorageSlot> {
            fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                &self,
                heap: &mut H,
                gas: &graph::runtime::gas::GasCounter,
            ) -> Result<AscStorageSlotArray, graph::runtime::HostExportError> {
                let content: Result<Vec<_>, _> = self
                    .iter()
                    .map(|x| graph::runtime::asc_new(heap, x, gas))
                    .collect();
                Ok(
                    AscStorageSlotArray(
                        graph_runtime_wasm::asc_abi::class::Array::new(
                            &content?,
                            heap,
                            gas,
                        )?,
                    ),
                )
            }
        }
        impl graph::runtime::AscType for AscStorageSlotArray {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                self.0.to_asc_bytes()
            }
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                Ok(
                    Self(
                        graph_runtime_wasm::asc_abi::class::Array::from_asc_bytes(
                            asc_obj,
                            api_version,
                        )?,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscStorageSlotArray {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelStorageSlotArray;
        }
        #[automatically_derived]
        mod __storageslot__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscStorageSlot> for StorageSlot {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscStorageSlot, graph::runtime::HostExportError> {
                    Ok(AscStorageSlot {
                        key: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.key),
                            gas,
                        )?,
                        value: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.value),
                            gas,
                        )?,
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscStorageSlot {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelStorageSlot;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscStorageSlot {
            pub key: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub value: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscStorageSlot {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "AscStorageSlot",
                    "key",
                    &&self.key,
                    "value",
                    &&self.value,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscStorageSlot {
            #[inline]
            fn default() -> AscStorageSlot {
                AscStorageSlot {
                    key: ::core::default::Default::default(),
                    value: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscStorageSlot {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.key.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.value.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscStorageSlot")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let key = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let value = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self { key, value })
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct UtxoId {
            #[prost(bytes = "vec", tag = "1")]
            pub tx_id: ::prost::alloc::vec::Vec<u8>,
            #[prost(uint32, tag = "2")]
            pub output_index: u32,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for UtxoId {
            #[inline]
            fn clone(&self) -> UtxoId {
                UtxoId {
                    tx_id: ::core::clone::Clone::clone(&self.tx_id),
                    output_index: ::core::clone::Clone::clone(&self.output_index),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for UtxoId {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for UtxoId {
            #[inline]
            fn eq(&self, other: &UtxoId) -> bool {
                self.tx_id == other.tx_id && self.output_index == other.output_index
            }
        }
        impl ::prost::Message for UtxoId {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if self.tx_id != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(1u32, &self.tx_id, buf);
                }
                if self.output_index != 0u32 {
                    ::prost::encoding::uint32::encode(2u32, &self.output_index, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "UtxoId";
                match tag {
                    1u32 => {
                        let mut value = &mut self.tx_id;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "tx_id");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.output_index;
                        ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "output_index");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.tx_id != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(1u32, &self.tx_id)
                    } else {
                        0
                    }
                    + if self.output_index != 0u32 {
                        ::prost::encoding::uint32::encoded_len(2u32, &self.output_index)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.tx_id.clear();
                self.output_index = 0u32;
            }
        }
        impl ::core::default::Default for UtxoId {
            fn default() -> Self {
                UtxoId {
                    tx_id: ::core::default::Default::default(),
                    output_index: 0u32,
                }
            }
        }
        impl ::core::fmt::Debug for UtxoId {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("UtxoId");
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.tx_id)
                    };
                    builder.field("tx_id", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.output_index)
                    };
                    builder.field("output_index", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        mod __utxoid__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscUtxoId> for UtxoId {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscUtxoId, graph::runtime::HostExportError> {
                    Ok(AscUtxoId {
                        tx_id: graph::runtime::asc_new(
                            heap,
                            &graph_runtime_wasm::asc_abi::class::Bytes(&self.tx_id),
                            gas,
                        )?,
                        output_index: self.output_index,
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscUtxoId {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelUtxoId;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscUtxoId {
            pub tx_id: graph::runtime::AscPtr<
                graph_runtime_wasm::asc_abi::class::Uint8Array,
            >,
            pub output_index: u32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscUtxoId {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "AscUtxoId",
                    "tx_id",
                    &&self.tx_id,
                    "output_index",
                    &&self.output_index,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscUtxoId {
            #[inline]
            fn default() -> AscUtxoId {
                AscUtxoId {
                    tx_id: ::core::default::Default::default(),
                    output_index: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscUtxoId {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.tx_id.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.output_index.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscUtxoId")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<
                        graph_runtime_wasm::asc_abi::class::Uint8Array,
                    >,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let tx_id = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u32>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let output_index = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self { tx_id, output_index })
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct TxPointer {
            #[prost(uint32, tag = "1")]
            pub block_height: u32,
            #[prost(uint32, tag = "2")]
            pub tx_index: u32,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for TxPointer {
            #[inline]
            fn clone(&self) -> TxPointer {
                TxPointer {
                    block_height: ::core::clone::Clone::clone(&self.block_height),
                    tx_index: ::core::clone::Clone::clone(&self.tx_index),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TxPointer {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for TxPointer {
            #[inline]
            fn eq(&self, other: &TxPointer) -> bool {
                self.block_height == other.block_height
                    && self.tx_index == other.tx_index
            }
        }
        impl ::prost::Message for TxPointer {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                if self.block_height != 0u32 {
                    ::prost::encoding::uint32::encode(1u32, &self.block_height, buf);
                }
                if self.tx_index != 0u32 {
                    ::prost::encoding::uint32::encode(2u32, &self.tx_index, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "TxPointer";
                match tag {
                    1u32 => {
                        let mut value = &mut self.block_height;
                        ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "block_height");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.tx_index;
                        ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "tx_index");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.block_height != 0u32 {
                        ::prost::encoding::uint32::encoded_len(1u32, &self.block_height)
                    } else {
                        0
                    }
                    + if self.tx_index != 0u32 {
                        ::prost::encoding::uint32::encoded_len(2u32, &self.tx_index)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.block_height = 0u32;
                self.tx_index = 0u32;
            }
        }
        impl ::core::default::Default for TxPointer {
            fn default() -> Self {
                TxPointer {
                    block_height: 0u32,
                    tx_index: 0u32,
                }
            }
        }
        impl ::core::fmt::Debug for TxPointer {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("TxPointer");
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.block_height)
                    };
                    builder.field("block_height", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.tx_index)
                    };
                    builder.field("tx_index", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        mod __txpointer__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscTxPointer> for TxPointer {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscTxPointer, graph::runtime::HostExportError> {
                    Ok(AscTxPointer {
                        block_height: self.block_height,
                        tx_index: self.tx_index,
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscTxPointer {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelTxPointer;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscTxPointer {
            pub block_height: u32,
            pub tx_index: u32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscTxPointer {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "AscTxPointer",
                    "block_height",
                    &&self.block_height,
                    "tx_index",
                    &&self.tx_index,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscTxPointer {
            #[inline]
            fn default() -> AscTxPointer {
                AscTxPointer {
                    block_height: ::core::default::Default::default(),
                    tx_index: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscTxPointer {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.block_height.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.tx_index.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscTxPointer")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u32>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let block_height = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                let field_align = std::mem::align_of::<u32>();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<u32>();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let tx_index = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self { block_height, tx_index })
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct Policies {
            #[prost(uint64, repeated, tag = "1")]
            pub values: ::prost::alloc::vec::Vec<u64>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Policies {
            #[inline]
            fn clone(&self) -> Policies {
                Policies {
                    values: ::core::clone::Clone::clone(&self.values),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Policies {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Policies {
            #[inline]
            fn eq(&self, other: &Policies) -> bool {
                self.values == other.values
            }
        }
        impl ::prost::Message for Policies {
            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::prost::bytes::BufMut,
            {
                ::prost::encoding::uint64::encode_packed(1u32, &self.values, buf);
            }
            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError>
            where
                B: ::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = "Policies";
                match tag {
                    1u32 => {
                        let mut value = &mut self.values;
                        ::prost::encoding::uint64::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "values");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::prost::encoding::uint64::encoded_len_packed(1u32, &self.values)
            }
            fn clear(&mut self) {
                self.values.clear();
            }
        }
        impl ::core::default::Default for Policies {
            fn default() -> Self {
                Policies {
                    values: ::prost::alloc::vec::Vec::new(),
                }
            }
        }
        impl ::core::fmt::Debug for Policies {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Policies");
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<u64>);
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                let mut vec_builder = f.debug_list();
                                for v in self.0 {
                                    fn Inner<T>(v: T) -> T {
                                        v
                                    }
                                    vec_builder.entry(&Inner(v));
                                }
                                vec_builder.finish()
                            }
                        }
                        ScalarWrapper(&self.values)
                    };
                    builder.field("values", &wrapper)
                };
                builder.finish()
            }
        }
        #[automatically_derived]
        mod __policies__ {
            use super::*;
            use crate::protobuf::*;
            impl graph::runtime::ToAscObj<AscPolicies> for Policies {
                #[allow(unused_variables)]
                fn to_asc_obj<H: graph::runtime::AscHeap + ?Sized>(
                    &self,
                    heap: &mut H,
                    gas: &graph::runtime::gas::GasCounter,
                ) -> Result<AscPolicies, graph::runtime::HostExportError> {
                    Ok(AscPolicies {
                        values: graph::runtime::asc_new(heap, &self.values, gas)?,
                        ..Default::default()
                    })
                }
            }
        }
        #[automatically_derived]
        impl graph::runtime::AscIndexId for AscPolicies {
            const INDEX_ASC_TYPE_ID: graph::runtime::IndexForAscTypeId = graph::runtime::IndexForAscTypeId::FuelPolicies;
        }
        #[automatically_derived]
        #[repr(C)]
        pub struct AscPolicies {
            pub values: graph::runtime::AscPtr<crate::protobuf::Ascu64Array>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AscPolicies {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AscPolicies",
                    "values",
                    &&self.values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AscPolicies {
            #[inline]
            fn default() -> AscPolicies {
                AscPolicies {
                    values: ::core::default::Default::default(),
                }
            }
        }
        impl graph::runtime::AscType for AscPolicies {
            fn to_asc_bytes(
                &self,
            ) -> Result<Vec<u8>, graph::runtime::DeterministicHostError> {
                let in_memory_byte_count = std::mem::size_of::<Self>();
                let mut bytes = Vec::with_capacity(in_memory_byte_count);
                let mut offset = 0;
                let mut max_align = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::Ascu64Array>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                    offset += padding_size;
                }
                let field_bytes = self.values.to_asc_bytes()?;
                bytes.extend_from_slice(&field_bytes);
                offset += field_bytes.len();
                if max_align < field_align {
                    max_align = field_align;
                }
                let struct_misalignment = offset % max_align;
                if struct_misalignment > 0 {
                    let padding_size = max_align - struct_misalignment;
                    bytes.extend_from_slice(&::alloc::vec::from_elem(0, padding_size));
                }
                match (&bytes.len(), &in_memory_byte_count) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Alignment mismatch for ",
                                            ", re-order fields or explicitely add a _padding field",
                                        ],
                                        &[::core::fmt::ArgumentV1::new_display(&"AscPolicies")],
                                    ),
                                ),
                            );
                        }
                    }
                };
                Ok(bytes)
            }
            #[allow(unused_variables)]
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &graph::semver::Version,
            ) -> Result<Self, graph::runtime::DeterministicHostError> {
                match api_version {
                    api_version if *api_version
                        <= graph::semver::Version::new(0, 0, 4) => {
                        if asc_obj.len() < std::mem::size_of::<Self>() {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                    _ => {
                        let content_size = std::mem::size_of::<Self>();
                        let aligned_size = graph::runtime::padding_to_16(content_size);
                        if graph::runtime::HEADER_SIZE + asc_obj.len()
                            == aligned_size + content_size
                        {
                            return Err(
                                graph::runtime::DeterministicHostError::from(
                                    ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Size does not match"],
                                                &[],
                                            ),
                                        );
                                        error
                                    }),
                                ),
                            );
                        }
                    }
                };
                let mut offset = 0;
                let field_align = std::mem::align_of::<
                    graph::runtime::AscPtr<crate::protobuf::Ascu64Array>,
                >();
                let misalignment = offset % field_align;
                if misalignment > 0 {
                    let padding_size = field_align - misalignment;
                    offset += padding_size;
                }
                let field_size = std::mem::size_of::<
                    graph::runtime::AscPtr<crate::protobuf::Ascu64Array>,
                >();
                let field_data = asc_obj
                    .get(offset..(offset + field_size))
                    .ok_or_else(|| {
                        graph::runtime::DeterministicHostError::from(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Attempted to read past end of array"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        )
                    })?;
                let values = graph::runtime::AscType::from_asc_bytes(
                    &field_data,
                    api_version,
                )?;
                offset += field_size;
                Ok(Self { values })
            }
        }
    }
    pub use graph_runtime_wasm::asc_abi::class::{Array, AscEnum, AscString, Uint8Array};
    pub use crate::runtime::abi::*;
    pub use pbcodec::*;
}
pub mod runtime {
    pub mod abi {
        use std::u64;
        use crate::protobuf::*;
        use graph::runtime::{
            asc_new, gas::GasCounter, AscHeap, AscIndexId, AscPtr, AscType,
            DeterministicHostError, HostExportError, IndexForAscTypeId, ToAscObj,
        };
        pub use graph::semver::Version;
        use graph_runtime_wasm::asc_abi::class::{ArrayBuffer, TypedArray};
        pub struct AscBytesArray(pub Array<AscPtr<Uint8Array>>);
        pub struct Ascu64Array(pub Array<AscPtr<TypedArray<u64>>>);
        impl AscIndexId for Block {
            const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::FuelBlock;
        }
        impl AscIndexId for Transaction {
            const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::FuelTransaction;
        }
        impl AscType for AscBytesArray {
            fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
                self.0.to_asc_bytes()
            }
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &Version,
            ) -> Result<Self, DeterministicHostError> {
                Ok(Self(Array::from_asc_bytes(asc_obj, api_version)?))
            }
        }
        impl AscIndexId for AscBytesArray {
            const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::FuelBytesArray;
        }
        impl ToAscObj<AscBytesArray> for Vec<Vec<u8>> {
            fn to_asc_obj<H: AscHeap + ?Sized>(
                &self,
                heap: &mut H,
                gas: &GasCounter,
            ) -> Result<AscBytesArray, HostExportError> {
                let content: Result<Vec<_>, _> = self
                    .iter()
                    .map(|x| asc_new(
                        heap,
                        &graph_runtime_wasm::asc_abi::class::Bytes(x),
                        gas,
                    ))
                    .collect();
                Ok(AscBytesArray(Array::new(&content?, heap, gas)?))
            }
        }
        impl ToAscObj<Ascu64Array> for Vec<Vec<u64>> {
            fn to_asc_obj<H: AscHeap + ?Sized>(
                &self,
                heap: &mut H,
                gas: &GasCounter,
            ) -> Result<Ascu64Array, HostExportError> {
                ::core::panicking::panic("not yet implemented")
            }
        }

        impl AscType for Ascu64Array {
            fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
                self.0.to_asc_bytes()
            }
            fn from_asc_bytes(
                asc_obj: &[u8],
                api_version: &Version,
            ) -> Result<Self, DeterministicHostError> {
                Ok(Self(Array::from_asc_bytes(asc_obj, api_version)?))
            }
        }
        impl AscIndexId for Ascu64Array {
            const INDEX_ASC_TYPE_ID: IndexForAscTypeId = IndexForAscTypeId::FuelU64Array;
        }
    }
}
mod trigger {
    use crate::codec;
    use graph::{
        blockchain::{Block, MappingTriggerTrait, TriggerData},
        prelude::{web3::types::H256, BlockNumber, CheapClone},
    };
    use std::{cmp::Ordering, sync::Arc};
    use graph::runtime::{asc_new, AscHeap, AscPtr, HostExportError};
    use graph::runtime::gas::GasCounter;
    use graph_runtime_wasm::module::ToAscPtr;
    pub enum FuelTrigger {
        Block(Arc<codec::Block>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FuelTrigger {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                FuelTrigger::Block(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Block",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FuelTrigger {
        #[inline]
        fn clone(&self) -> FuelTrigger {
            match self {
                FuelTrigger::Block(__self_0) => {
                    FuelTrigger::Block(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl FuelTrigger {
        pub fn block_number(&self) -> BlockNumber {
            match self {
                FuelTrigger::Block(block) => block.number(),
            }
        }
        pub fn block_hash(&self) -> H256 {
            match self {
                FuelTrigger::Block(block) => block.ptr().hash_as_h256(),
            }
        }
        fn error_context(&self) -> String {
            match self {
                FuelTrigger::Block(..) => {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["Block #", " (", ")"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&self.block_number()),
                                ::core::fmt::ArgumentV1::new_display(&self.block_hash()),
                            ],
                        ),
                    );
                    res
                }
            }
        }
    }
    impl CheapClone for FuelTrigger {
        fn cheap_clone(&self) -> FuelTrigger {
            match self {
                FuelTrigger::Block(block) => FuelTrigger::Block(block.cheap_clone()),
            }
        }
    }
    impl ToAscPtr for FuelTrigger {
        fn to_asc_ptr<H: AscHeap>(
            self,
            heap: &mut H,
            gas: &GasCounter,
        ) -> Result<AscPtr<()>, HostExportError> {
            Ok(
                match self {
                    FuelTrigger::Block(block) => {
                        asc_new(heap, block.as_ref(), gas)?.erase()
                    }
                },
            )
        }
    }
    impl TriggerData for FuelTrigger {
        fn error_context(&self) -> String {
            self.error_context()
        }
        fn address_match(&self) -> Option<&[u8]> {
            None
        }
    }
    impl PartialEq for FuelTrigger {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Block(a_ptr), Self::Block(b_ptr)) => a_ptr == b_ptr,
            }
        }
    }
    impl PartialOrd for FuelTrigger {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Eq for FuelTrigger {}
    impl Ord for FuelTrigger {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self, other) {
                (Self::Block(..), Self::Block(..)) => Ordering::Equal,
                (Self::Block(..), _) => Ordering::Greater,
                (_, Self::Block(..)) => Ordering::Less,
            }
        }
    }
    impl MappingTriggerTrait for FuelTrigger {
        fn error_context(&self) -> String {
            self.error_context()
        }
    }
}
pub use crate::{adapter::TriggerFilter, chain::Chain};
pub use protobuf::{pbcodec, pbcodec::Block};
