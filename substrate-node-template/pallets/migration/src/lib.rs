#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// 方法一打开这个
// pub mod migration;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, traits::StorageVersion};
	use frame_system::pallet_prelude::*;

	// use frame_support::traits::OnRuntimeUpgrade;
	use frame_support::{
		traits::{Get, GetStorageVersion},
		weights::Weight,
	};
	use sp_runtime::traits::Saturating;

	// const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	// pub type Something<T> = StorageValue<_, u32>;
	pub type Something<T> = StorageValue<_, (u32, u32)>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// <Something<T>>::put(something);
			<Something<T>>::put((something, 1));
			Self::deposit_event(Event::SomethingStored(something, who));
			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// 方法二
		fn on_runtime_upgrade() -> Weight {
			let current = Pallet::<T>::current_storage_version();
			let onchain = Pallet::<T>::on_chain_storage_version();

			log::info!(
				target: "migration",
				"Running migration with current storage version {:?} / onchain {:?}",
				current,
				onchain
			);

			if current == 1 && onchain == 0 {
				let mut translated = 0u64;

				let _ = Something::<T>::translate::<u32, _>(|old_value| {
					translated.saturating_inc();
					match old_value {
						Some(v) => Some((v, 0u32)),
						None => Some((0u32, 0u32)),
					}
				});

				log::info!(
					target: "migration",
					"updating to version 2 +++++++++++++++++++++++++++++++++++++++++++++++",
				);

				current.put::<Pallet<T>>();
				T::DbWeight::get().reads_writes(translated + 1, translated + 1)
			} else {
				log::info!(
					target: "migration",
					"InjectValidatorsIntoVoterList being executed on the wrong storage \
				version, expected Releases::V2_0_0"
				);
				// 0
				T::DbWeight::get().reads(1)
			}
		}

		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<(), &'static str> {
			use frame_support::traits::OnRuntimeUpgradeHelpersExt;
			frame_support::ensure!(
				StorageVersion::<T>::get() == crate::Releases::V1_0_0,
				"must upgrade linearly"
			);

			Ok(())
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade() -> Result<(), &'static str> {
			assert_eq!(Pallet::<T>::on_chain_storage_version(), 1);
			Ok(())
		}
	}
}
