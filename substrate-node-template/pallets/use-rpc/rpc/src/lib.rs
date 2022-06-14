use std::sync::Arc;

pub use self::gen_client::Client as UseRpcClient;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use use_rpc_runtime_api::MyRpcRuntimeApi;

pub struct UseRpc<C, B> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<B>,
}

impl<C, B> UseRpc<C, B> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

#[rpc]
pub trait MyRpcApi<BlockHash> {
	#[rpc(name = "my_rpc_method")]
	fn rpc_method(&self, v: u32, at: Option<BlockHash>) -> Result<bool>;
}

impl<C, Block> MyRpcApi<<Block as BlockT>::Hash> for UseRpc<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: MyRpcRuntimeApi<Block>,
{
	fn rpc_method(&self, v: u32, at: Option<<Block as BlockT>::Hash>) -> Result<bool> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		let runtime_api_result = api.rpc_method(&at, v);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876),
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}
