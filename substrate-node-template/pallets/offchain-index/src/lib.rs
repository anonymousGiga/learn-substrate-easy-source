#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;

	use sp_runtime::offchain::storage::StorageValueRef;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_block_number: T::BlockNumber) {
			if let Ok(some_number) = Self::get_local_storage() {
				log::info!(target:"offchain-index-demo", "1. Offchain-index, some_number ======================== {:?}", some_number);
			} else {
				log::info!(target:"offchain-index-demo", "2. Offchain-index, no number in storage ==================== ");
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_local_storage(
			origin: OriginFor<T>,
			some_number: u32,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			Self::set_local_storage_with_offchain_index(some_number);
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn derived_key() -> Vec<u8> {
			b"offchain-index-demo::value".encode()
		}

		fn set_local_storage_with_offchain_index(some_number: u32) {
			let key = Self::derived_key();
			sp_io::offchain_index::set(&key, some_number.encode().as_slice());
			log::info!(target:"offchain-index-demo", "set some_number ======================== {:?}", some_number);
		}

		fn get_local_storage() -> Result<u32, &'static str> {
			let key = Self::derived_key();
			let some_number_storage = StorageValueRef::persistent(&key);

			if let Ok(Some(number)) = some_number_storage.get::<u32>() {
				Ok(number)
			} else {
				Err("No number in storage.")
			}
		}
	}
}
