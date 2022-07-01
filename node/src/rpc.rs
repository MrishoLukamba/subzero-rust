//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use jsonrpsee::RpcModule;
use node_template_runtime::{opaque::Block, AccountId, Balance, Index, Hash, BlockNumber};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sc_consensus_babe::{Config, Epoch};
use sc_consensus_epochs::SharedEpochChanges;
use sc_finality_grandpa::{
	SharedVoterState, SharedAuthoritySet, FinalityProofProvider, GrandpaJustificationStream
};
use sc_keystore::KeyStorePtr;
use sc_rpc::SubscriptionTaskExecutor;
use sp_consensus::SelectChain;
use sp_consensus_babe::BabeApi;
pub use sc_rpc_api::DenyUnsafe;

pub struct BabeDeps {
	/// BABE protocol config.
	pub babe_config: Config,
	/// BABE pending epoch changes.
	pub shared_epoch_changes: SharedEpochChanges<Block, Epoch>,
	/// The keystore that manages the keys of the node.
	pub keystore: KeyStorePtr,
}

pub struct GrandpaDeps<B> {
	/// Voting round info.
	pub shared_voter_state: SharedVoterState,
	/// Authority set info.
	pub shared_authority_set: SharedAuthoritySet<Hash, BlockNumber>,
	/// Receives notifications about justification events from Grandpa.
	pub justification_stream: GrandpaJustificationStream<Block>,
	/// Executor to drive the subscription manager in the Grandpa RPC handler.
	pub subscription_executor: SubscriptionTaskExecutor,
	/// Finality proof provider.
	pub finality_provider: Arc<FinalityProofProvider<B, Block>>,
}


/// Full client dependencies.
pub struct FullDeps<C, P, SC, B> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// The SelectChain Strategy
	pub select_chain: SC,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// BABE specific dependencies.
	pub babe: BabeDeps,
	/// GRANDPA specific dependencies.
	pub grandpa: GrandpaDeps<B>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, SC, B>(
	deps: FullDeps<C, P, SC, B>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	C::APi: BabeApi<Block>,
	P: TransactionPool + 'static,
	SC: SelectChain<Block> + 'static',
	B: sc_client_api::Backend<Block> + Send + Sync + 'static',
	B::state: sc_client_api::StateBackend<sp_runtime::traits::HashFor<Block>>;

{
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	let FullDeps{
    		client,
    		pool,
    		select_chain,
    		deby_unsafe,
    		babe,
    		grandpa,
    	} = deps;

    	let BabeDeps {
        		keystore,
        		babe_config,
        		shared_epoch_changes,
        	} = babe;

        let GrandpaDeps {
        		shared_voter_state,
        		shared_authority_set,
        		justification_stream,
        		subscription_executor,
        		finality_provider,
        	} = grandpa;

	module.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client).into_rpc())?;
	module.merge(
    		Babe::new(
    			client.clone(),
    			shared_epoch_changes.clone(),
    			keystore,
    			babe_config,
    			select_chain,
    			deny_unsafe,
    		)
    		.into_rpc(),
    	)?;
    module.merge(
    		Grandpa::new(
    			subscription_executor,
    			shared_authority_set.clone(),
    			shared_voter_state,
    			justification_stream,
    			finality_provider,
    		)
    		.into_rpc(),
    	)?;
	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `module.merge(YourRpcTrait::into_rpc(YourRpcStruct::new(ReferenceToClient, ...)))?;`

	Ok(module)
}
