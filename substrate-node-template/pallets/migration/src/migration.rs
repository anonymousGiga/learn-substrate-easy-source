use super::*;
use frame_support::traits::OnRuntimeUpgrade;
use frame_support::{
	traits::{Get, GetStorageVersion},
	weights::Weight,
};
use sp_runtime::traits::Saturating;

// 方法一
// pub mod v2 {
	// use super::*;

    // pub struct MigrateToV2<T>(sp_std::marker::PhantomData<T>);
	// impl<T: Config> OnRuntimeUpgrade for MigrateToV2<T> {
	// 	fn on_runtime_upgrade() -> Weight {
	// 		let current = Pallet::<T>::current_storage_version();
	// 		let onchain = Pallet::<T>::on_chain_storage_version();

	// 		log::info!(
	// 			target: "migration",
	// 			"Running migration with current storage version {:?} / onchain {:?}",
	// 			current,
	// 			onchain
	// 		);

	// 		if current == 1 && onchain == 0 {
	// 			let mut translated = 0u64;

	// 			let _ = Something::<T>::translate::<u32, _>(|old_value| {
	// 				translated.saturating_inc();
	// 				match old_value {
	// 					Some(v) => Some((v, 0u32)),
	// 					None => Some((0u32, 0u32)),
	// 				}
	// 			});


	// 			log::info!(
	// 				target: "migration",
	// 				"updating to version 2 +++++++++++++++++++++++++++++++++++++++++++++++",
	// 			);

	// 			current.put::<Pallet<T>>();
	// 			T::DbWeight::get().reads_writes(translated + 1, translated + 1)
	// 		} else {
	// 			log::info!(
	// 				target: "migration",
	// 				"InjectValidatorsIntoVoterList being executed on the wrong storage \
	// 			version, expected Releases::V2_0_0"
	// 			);
    //             // 0
	// 			T::DbWeight::get().reads(1)
	// 		}
	// 	}

	// 	#[cfg(feature = "try-runtime")]
	// 	fn pre_upgrade() -> Result<(), &'static str> {
	// 		use frame_support::traits::OnRuntimeUpgradeHelpersExt;
	// 		frame_support::ensure!(
	// 			StorageVersion::<T>::get() == crate::Releases::V1_0_0,
	// 			"must upgrade linearly"
	// 		);

	// 		Ok(())
	// 	}

	// 	#[cfg(feature = "try-runtime")]
	// 	fn post_upgrade() -> Result<(), &'static str> {
	// 		assert_eq!(Pallet::<T>::on_chain_storage_version(), 1);
	// 		Ok(())
	// 	}
	// }
// }
