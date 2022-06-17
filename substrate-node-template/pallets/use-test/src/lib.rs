#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// 1. Imports and Dependencies
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use codec::Codec;
	use frame_support::{
		pallet_prelude::*, sp_runtime::traits::AtLeast32BitUnsigned, sp_std::fmt::Debug,
	};
	use frame_system::pallet_prelude::*;

	// 2. Declaration of the Pallet type
	// This is a placeholder to implement traits and methods.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// 3. Runtime Configuration Trait
	// All types and constants go here.
	// Use #[pallet::constant] and #[pallet::extra_constants]
	// to pass in values to metadata.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type ClassType: Member
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Codec
			+ Copy
			+ Debug
			+ Default
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize;
	}

	// 4. Runtime Storage
	// use storageValue store class.
	#[pallet::storage]
	#[pallet::getter(fn my_class)]
	pub type Class<T: Config> = StorageValue<_, T::ClassType, ValueQuery>;

	// 5. Runtime Events
	// Can stringify event types to metadata.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetClass(T::ClassType),
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub class: T::ClassType,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { class: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			Class::<T>::put(self.class);
		}
	}

	// 7. Extrinsics
	// Functions that are callable from outside the runtime.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_class_info(
			origin: OriginFor<T>,
			class: T::ClassType,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			Class::<T>::put(class);
			Self::deposit_event(Event::SetClass(class));

			Ok(().into())
		}
	}
}
