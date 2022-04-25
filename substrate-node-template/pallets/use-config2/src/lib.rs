#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
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

	// 3. Runtime Configuration Trait
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		//声明StudentNumber类型
		type StudentNumberType: Member
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Codec
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
		#[pallet::weight(0)]
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

			Ok(().into())
		}
	}
}
