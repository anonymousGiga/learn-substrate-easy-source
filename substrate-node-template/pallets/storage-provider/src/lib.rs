#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use traits::StorageInterface;

pub mod traits;

#[frame_support::pallet]
pub mod pallet {
	use codec::Codec;
	use frame_support::{
		pallet_prelude::*, sp_runtime::traits::AtLeast32BitUnsigned, sp_std::fmt::Debug,
	};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

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
	}

	#[pallet::storage]
	pub type MyValue<T: Config> = StorageValue<_, T::Value, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		FunctionCall,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn my_function(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;
			log::info!(target: "storage provider", "my function!");
			Self::deposit_event(Event::FunctionCall);

			Ok(().into())
		}
	}
}

impl<T: Config> StorageInterface for Pallet<T> {
	type Value = T::Value;

	fn get_param() -> Self::Value {
		MyValue::<T>::get()
	}

	fn set_param(v: Self::Value) {
		MyValue::<T>::put(v);
	}
}
