use std::sync::Arc;

pub use self::gen_client::Client as UseRpcClient;
use codec::{Codec, Encode};
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
use serde::{Deserialize, Serialize};

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};

// An implementation of UseRpc specific RPC methods.
pub struct UseRpc<C, B> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<B>,
}

impl<C, B> UseRpc<C, B> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// RPC methods.
#[rpc]
pub trait MyRpcApi<BlockHash> {
	#[rpc(name = "my_rpc_method")]
	fn rpc_method(&self, v: u32) -> Result<bool>;
}

impl<C, Block> MyRpcApi<<Block as BlockT>::Hash> for UseRpc<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
{
	fn rpc_method(&self, _v: u32) -> Result<bool> {
		Ok(true)
	}
}
