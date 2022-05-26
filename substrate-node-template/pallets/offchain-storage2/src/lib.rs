#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	extern crate alloc;
	use alloc::string::{String, ToString};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			let flag = Self::should_print();

			if flag {
				log::info!(target:"offchain-storage2222", "1. Offchain-storage2, block number: {:?}, should print +++++++++++ ", block_number);
			} else {
				log::info!(target:"offchain-storage2222", "2. Offchain-storage2, block number: {:?}, Should not print +++++++++++ ", block_number);
			}

			Self::set_result(flag);
		}
	}

	impl<T: Config> Pallet<T> {
		fn should_print() -> bool {
			let kind = sp_core::offchain::StorageKind::PERSISTENT;
			if let Some(flag) = sp_io::offchain::local_storage_get(kind, b"demo::flag") {
				let ret = match String::from_utf8(flag) {
					Ok(v) => v.eq(&"true"),
					Err(_) => false,
				};

				ret
			} else {
				false
			}
		}

		fn set_result(flag: bool) {
			let kind = sp_core::offchain::StorageKind::PERSISTENT;
			let value = match flag {
				true => "true".to_string(),
				false => "false".to_string(),
			};
			sp_io::offchain::local_storage_set(kind, b"demo::result", value.as_bytes());
		}
	}
}
