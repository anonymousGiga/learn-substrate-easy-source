#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports and Dependencies
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
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
	}

	// 4. Runtime Storage
	// use storageValue store class.
	#[pallet::storage]
	#[pallet::getter(fn my_class)]
	pub type Class<T: Config> = StorageValue<_, u32, ValueQuery>;

	// use storageMap store (student number -> student name).
	#[pallet::storage]
	#[pallet::getter(fn students_info)]
	pub type StudentsInfo<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn dorm_info)]
	pub type DormInfo<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u32, //dorm number
		Blake2_128Concat,
		u32, //bed number
		u32, // student number
		ValueQuery,
	>;

	// 5. Runtime Events
	// Can stringify event types to metadata.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetClass(u32),
		SetStudentInfo(u32, u128),
		SetDormInfo(u32, u32, u32),
	}

	// 7. Extrinsics
	// Functions that are callable from outside the runtime.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_class_info(origin: OriginFor<T>, class: u32) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			Class::<T>::put(class);
			Self::deposit_event(Event::SetClass(class));

			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn set_student_info(
			origin: OriginFor<T>,
			student_number: u32,
			student_name: u128,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			StudentsInfo::<T>::insert(&student_number, &student_name);
			Self::deposit_event(Event::SetStudentInfo(student_number, student_name));

			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn set_dorm_info(
			origin: OriginFor<T>,
			dorm_number: u32,
			bed_number: u32,
			student_number: u32,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			DormInfo::<T>::insert(&dorm_number, &bed_number, &student_number);
			Self::deposit_event(Event::SetDormInfo(dorm_number, bed_number, student_number));

			Ok(().into())
		}
	}
}
