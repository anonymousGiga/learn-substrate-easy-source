#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::Codec;
	use frame_support::{
		pallet_prelude::*, sp_runtime::traits::AtLeast32BitUnsigned, sp_std::fmt::Debug,
	};
	use frame_system::pallet_prelude::*;
	use pallet_storage_provider::traits::StorageInterface;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// 3. Runtime Configuration Trait
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Value: Member
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Codec
			+ From<u32>
			+ Into<u32>
			+ Copy
			+ Debug
			+ Default
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize;
		type MyStorage: StorageInterface<Value = Self::Value>;
	}

	// 5. Runtime Events
	// Can stringify event types to metadata.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		StoreEvent,
	}

	// 7. Extrinsics
	// Functions that are callable from outside the runtime.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn storage_value(origin: OriginFor<T>, value: T::Value) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			T::MyStorage::set_param(value);

			let v = T::MyStorage::get_param();
			log::info!(target: "other-pallet", "Value get from storage is: {:?}", v);

			Self::deposit_event(Event::StoreEvent);

			Ok(().into())
		}
	}
}
