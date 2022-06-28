#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

// #[cfg(test)]
// mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use codec::Codec;
	use frame_support::{
		pallet_prelude::*, sp_runtime::traits::AtLeast32BitUnsigned, sp_std::fmt::Debug,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;

	use crate::WeightInfo;
	use sp_io::hashing::{blake2_128, twox_128};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// 3. Runtime Configuration Trait
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		//声明StudentNumber类型
		type StudentNumberType: Member
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

		//声明StudentName类型
		type StudentNameType: Parameter
			+ Member
			+ AtLeast32BitUnsigned
			+ Codec
			+ Default
			+ From<u128>
			+ Into<u128>
			+ Copy
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ Debug;

		type WeightInfo: WeightInfo;
	}

	// 4. Runtime Storage
	// 用storageMap存储学生信息，（key， value）分别对应的是学号和姓名.
	#[pallet::storage]
	#[pallet::getter(fn students_info)]
	pub type StudentsInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, T::StudentNumberType, T::StudentNameType, ValueQuery>;

	// 5. Runtime Events
	// Can stringify event types to metadata.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetStudentInfo(T::StudentNumberType, T::StudentNameType),
	}

	// 8. Runtime Errors
	#[pallet::error]
	pub enum Error<T> {
		// 相同学号的只允许设置一次名字
		SetStudentsInfoDuplicate,
	}

	// 7. Extrinsics
	// Functions that are callable from outside the runtime.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(<T as Config>::WeightInfo::set_student_info((*student_number).into() ))]
		pub fn set_student_info(
			origin: OriginFor<T>,
			student_number: T::StudentNumberType,
			student_name: T::StudentNameType,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			if StudentsInfo::<T>::contains_key(student_number) {
				return Err(Error::<T>::SetStudentsInfoDuplicate.into())
			}

			StudentsInfo::<T>::insert(&student_number, &student_name);
			Self::deposit_event(Event::SetStudentInfo(student_number, student_name));

			Self::generate_key();

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn generate_key() {
			// let key1 = twox_128(b"Balances".to_vec().as_slice());
			let key1 = twox_128(b"Balances");
			let key2 = twox_128(b"FreeBalance");
			let key3 = Self::blake2_128_concat(
				b"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
			);
			log::info!(target: "use-benchmarking", "------------ ============================, key1 = {:?}, key2 = {:?}, key3 = {:?}", key1, key2, key3);

			let a = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".as_bytes().to_vec();
			a.using_encoded(|ref slice| {
				log::info!(target: "use-benchmarking", "------------ ============================, code = {:?}", slice);
			});
		}

		fn blake2_128_concat(d: &[u8]) -> Vec<u8> {
			let mut v = blake2_128(d).to_vec();
			v.extend_from_slice(d);
			v
		}
	}
}
