#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use sp_runtime::offchain::storage::{
		MutateStorageError, StorageRetrievalError, StorageValueRef,
	};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			if Self::should_print() {
				log::info!(target:"offchain-storage", "1. Block number: {:?}, should print +++++++++++++++++++++++++++ ", block_number);
			} else {
				log::info!(target:"offchain-storage", "2. Block number: {:?}, Should not print +++++++++++++++++++++++++++ ", block_number);
			}
		}
	}

	impl<T: Config> Pallet<T> {
		fn should_print() -> bool {
			const LAST_VALUE: () = ();

			let val = StorageValueRef::persistent(b"ocw-demo::last_value");
			let res =
				val.mutate(
					|last_flag: Result<Option<bool>, StorageRetrievalError>| match last_flag {
						Ok(Some(flag)) => Ok(!flag),
						_ => Ok(true),
					},
				);

			match res {
				Ok(flag) => flag,
				Err(MutateStorageError::ValueFunctionFailed(LAST_VALUE)) => false,
				Err(MutateStorageError::ConcurrentModification(_)) => false,
			}
		}
	}
}
