#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
	pub trait MyRpcRuntimeApi {
		fn rpc_method(v: u32) -> bool;
	}
}
