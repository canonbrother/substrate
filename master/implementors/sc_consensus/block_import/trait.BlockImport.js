(function() {var implementors = {
"beefy_gadget":[["impl&lt;Block, BE, Runtime, I&gt; BlockImport&lt;Block&gt; for <a class=\"struct\" href=\"beefy_gadget/import/struct.BeefyBlockImport.html\" title=\"struct beefy_gadget::import::BeefyBlockImport\">BeefyBlockImport</a>&lt;Block, BE, Runtime, I&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Block: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;BE: <a class=\"trait\" href=\"sc_client_api/backend/trait.Backend.html\" title=\"trait sc_client_api::backend::Backend\">Backend</a>&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: BlockImport&lt;Block, Error = ConsensusError, Transaction = TransactionFor&lt;Runtime, Block&gt;&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime: ProvideRuntimeApi&lt;Block&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime::Api: <a class=\"trait\" href=\"beefy_primitives/trait.BeefyApi.html\" title=\"trait beefy_primitives::BeefyApi\">BeefyApi</a>&lt;Block&gt;,</span>"]],
"sc_consensus":[],
"sc_consensus_babe":[["impl&lt;Block, Client, Inner&gt; BlockImport&lt;Block&gt; for <a class=\"struct\" href=\"sc_consensus_babe/struct.BabeBlockImport.html\" title=\"struct sc_consensus_babe::BabeBlockImport\">BabeBlockImport</a>&lt;Block, Client, Inner&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Block: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Inner: BlockImport&lt;Block, Transaction = TransactionFor&lt;Client, Block&gt;&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Inner::Error: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;ConsensusError&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Client: HeaderBackend&lt;Block&gt; + HeaderMetadata&lt;Block, Error = Error&gt; + <a class=\"trait\" href=\"sc_client_api/backend/trait.AuxStore.html\" title=\"trait sc_client_api::backend::AuxStore\">AuxStore</a> + ProvideRuntimeApi&lt;Block&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Client::Api: <a class=\"trait\" href=\"sc_consensus_babe/trait.BabeApi.html\" title=\"trait sc_consensus_babe::BabeApi\">BabeApi</a>&lt;Block&gt; + ApiExt&lt;Block&gt;,</span>"]],
"sc_consensus_pow":[["impl&lt;B, I, C, S, Algorithm, CIDP&gt; BlockImport&lt;B&gt; for <a class=\"struct\" href=\"sc_consensus_pow/struct.PowBlockImport.html\" title=\"struct sc_consensus_pow::PowBlockImport\">PowBlockImport</a>&lt;B, I, C, S, Algorithm, CIDP&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: BlockImport&lt;B, Transaction = TransactionFor&lt;C, B&gt;&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Error: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;ConsensusError&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: SelectChain&lt;B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: ProvideRuntimeApi&lt;B&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + HeaderBackend&lt;B&gt; + <a class=\"trait\" href=\"sc_client_api/backend/trait.AuxStore.html\" title=\"trait sc_client_api::backend::AuxStore\">AuxStore</a> + <a class=\"trait\" href=\"sc_client_api/client/trait.BlockOf.html\" title=\"trait sc_client_api::client::BlockOf\">BlockOf</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;C::Api: BlockBuilderApi&lt;B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Algorithm: <a class=\"trait\" href=\"sc_consensus_pow/trait.PowAlgorithm.html\" title=\"trait sc_consensus_pow::PowAlgorithm\">PowAlgorithm</a>&lt;B&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Algorithm::<a class=\"associatedtype\" href=\"sc_consensus_pow/trait.PowAlgorithm.html#associatedtype.Difficulty\" title=\"type sc_consensus_pow::PowAlgorithm::Difficulty\">Difficulty</a>: 'static + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;CIDP: CreateInherentDataProviders&lt;B, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</span>"]],
"sc_finality_grandpa":[["impl&lt;BE, Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>, Client, SC&gt; BlockImport&lt;Block&gt; for <a class=\"struct\" href=\"sc_finality_grandpa/struct.GrandpaBlockImport.html\" title=\"struct sc_finality_grandpa::GrandpaBlockImport\">GrandpaBlockImport</a>&lt;BE, Block, Client, SC&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"type\" href=\"sp_runtime/traits/type.NumberFor.html\" title=\"type sp_runtime::traits::NumberFor\">NumberFor</a>&lt;Block&gt;: <a class=\"trait\" href=\"sc_finality_grandpa/trait.BlockNumberOps.html\" title=\"trait sc_finality_grandpa::BlockNumberOps\">BlockNumberOps</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;BE: <a class=\"trait\" href=\"sc_client_api/backend/trait.Backend.html\" title=\"trait sc_client_api::backend::Backend\">Backend</a>&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Client: <a class=\"trait\" href=\"sc_finality_grandpa/trait.ClientForGrandpa.html\" title=\"trait sc_finality_grandpa::ClientForGrandpa\">ClientForGrandpa</a>&lt;Block, BE&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Client::Api: <a class=\"trait\" href=\"sc_finality_grandpa/trait.GrandpaApi.html\" title=\"trait sc_finality_grandpa::GrandpaApi\">GrandpaApi</a>&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;for&lt;'a&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a </a>Client: BlockImport&lt;Block, Error = ConsensusError, Transaction = TransactionFor&lt;Client, Block&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;TransactionFor&lt;Client, Block&gt;: 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;SC: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,</span>"]],
"sc_network_test":[["impl <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;<a class=\"struct\" href=\"sp_runtime/generic/block/struct.Block.html\" title=\"struct sp_runtime::generic::block::Block\">Block</a>&lt;<a class=\"struct\" href=\"sp_runtime/generic/header/struct.Header.html\" title=\"struct sp_runtime::generic::header::Header\">Header</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>, <a class=\"struct\" href=\"sp_runtime/traits/struct.BlakeTwo256.html\" title=\"struct sp_runtime::traits::BlakeTwo256\">BlakeTwo256</a>&gt;, <a class=\"enum\" href=\"sc_network_test/enum.Extrinsic.html\" title=\"enum sc_network_test::Extrinsic\">Extrinsic</a>&gt;&gt; for <a class=\"struct\" href=\"sc_network_test/struct.PeersClient.html\" title=\"struct sc_network_test::PeersClient\">PeersClient</a>"],["impl&lt;I, Transaction&gt; <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;<a class=\"struct\" href=\"sp_runtime/generic/block/struct.Block.html\" title=\"struct sp_runtime::generic::block::Block\">Block</a>&lt;<a class=\"struct\" href=\"sp_runtime/generic/header/struct.Header.html\" title=\"struct sp_runtime::generic::header::Header\">Header</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>, <a class=\"struct\" href=\"sp_runtime/traits/struct.BlakeTwo256.html\" title=\"struct sp_runtime::traits::BlakeTwo256\">BlakeTwo256</a>&gt;, <a class=\"enum\" href=\"sc_network_test/enum.Extrinsic.html\" title=\"enum sc_network_test::Extrinsic\">Extrinsic</a>&gt;&gt; for <a class=\"struct\" href=\"sc_network_test/struct.BlockImportAdapter.html\" title=\"struct sc_network_test::BlockImportAdapter\">BlockImportAdapter</a>&lt;I, Transaction&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;<a class=\"type\" href=\"sc_network_test/type.Block.html\" title=\"type sc_network_test::Block\">Block</a>, Error = <a class=\"enum\" href=\"sp_consensus/error/enum.Error.html\" title=\"enum sp_consensus::error::Error\">ConsensusError</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::<a class=\"associatedtype\" href=\"sc_consensus/block_import/trait.BlockImport.html#associatedtype.Transaction\" title=\"type sc_consensus::block_import::BlockImport::Transaction\">Transaction</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Transaction: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,</span>"]],
"sc_service":[["impl&lt;'impl0, B, E, Block, RA&gt; <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;Block&gt; for &amp;'impl0 <a class=\"struct\" href=\"sc_service/client/struct.Client.html\" title=\"struct sc_service::client::Client\">Client</a>&lt;B, E, Block, RA&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"sc_client_api/backend/trait.Backend.html\" title=\"trait sc_client_api::backend::Backend\">Backend</a>&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"sc_client_api/call_executor/trait.CallExecutor.html\" title=\"trait sc_client_api::call_executor::CallExecutor\">CallExecutor</a>&lt;Block&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Block: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"sc_service/client/struct.Client.html\" title=\"struct sc_service::client::Client\">Client</a>&lt;B, E, Block, RA&gt;: <a class=\"trait\" href=\"sp_api/trait.ProvideRuntimeApi.html\" title=\"trait sp_api::ProvideRuntimeApi\">ProvideRuntimeApi</a>&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"sc_service/client/struct.Client.html\" title=\"struct sc_service::client::Client\">Client</a>&lt;B, E, Block, RA&gt; as <a class=\"trait\" href=\"sp_api/trait.ProvideRuntimeApi.html\" title=\"trait sp_api::ProvideRuntimeApi\">ProvideRuntimeApi</a>&lt;Block&gt;&gt;::<a class=\"associatedtype\" href=\"sp_api/trait.ProvideRuntimeApi.html#associatedtype.Api\" title=\"type sp_api::ProvideRuntimeApi::Api\">Api</a>: <a class=\"trait\" href=\"sp_api/trait.Core.html\" title=\"trait sp_api::Core\">CoreApi</a>&lt;Block&gt; + <a class=\"trait\" href=\"sp_api/trait.ApiExt.html\" title=\"trait sp_api::ApiExt\">ApiExt</a>&lt;Block, StateBackend = B::<a class=\"associatedtype\" href=\"sc_client_api/backend/trait.Backend.html#associatedtype.State\" title=\"type sc_client_api::backend::Backend::State\">State</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;RA: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"type\" href=\"sc_client_api/backend/type.TransactionFor.html\" title=\"type sc_client_api::backend::TransactionFor\">TransactionFor</a>&lt;B, Block&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,</span>"],["impl&lt;B, E, Block, RA&gt; <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;Block&gt; for <a class=\"struct\" href=\"sc_service/client/struct.Client.html\" title=\"struct sc_service::client::Client\">Client</a>&lt;B, E, Block, RA&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"sc_client_api/backend/trait.Backend.html\" title=\"trait sc_client_api::backend::Backend\">Backend</a>&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"sc_client_api/call_executor/trait.CallExecutor.html\" title=\"trait sc_client_api::call_executor::CallExecutor\">CallExecutor</a>&lt;Block&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Block: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: <a class=\"trait\" href=\"sp_api/trait.ProvideRuntimeApi.html\" title=\"trait sp_api::ProvideRuntimeApi\">ProvideRuntimeApi</a>&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Self as <a class=\"trait\" href=\"sp_api/trait.ProvideRuntimeApi.html\" title=\"trait sp_api::ProvideRuntimeApi\">ProvideRuntimeApi</a>&lt;Block&gt;&gt;::<a class=\"associatedtype\" href=\"sp_api/trait.ProvideRuntimeApi.html#associatedtype.Api\" title=\"type sp_api::ProvideRuntimeApi::Api\">Api</a>: <a class=\"trait\" href=\"sp_api/trait.Core.html\" title=\"trait sp_api::Core\">CoreApi</a>&lt;Block&gt; + <a class=\"trait\" href=\"sp_api/trait.ApiExt.html\" title=\"trait sp_api::ApiExt\">ApiExt</a>&lt;Block, StateBackend = B::<a class=\"associatedtype\" href=\"sc_client_api/backend/trait.Backend.html#associatedtype.State\" title=\"type sc_client_api::backend::Backend::State\">State</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;RA: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"type\" href=\"sc_client_api/backend/type.TransactionFor.html\" title=\"type sc_client_api::backend::TransactionFor\">TransactionFor</a>&lt;B, Block&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()