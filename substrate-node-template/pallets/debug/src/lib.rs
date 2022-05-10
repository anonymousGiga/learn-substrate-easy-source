#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	// use sp_runtime::traits::Printable;
	use sp_runtime::print;
	use sp_std::if_std;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	// impl<T: Config> Printable for Error<T> {
    //     fn print(&self) {
    //         match self {
    //             Error::NoneValue => "Invalid Value".print(),
    //             Error::StorageOverflow => "++++++++++++++++++++++++++ Value Exceeded and Overflowed".print(),
    //             _ => "Invalid Error Case".print(),
    //         }
    //     }
    // }


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Something<T>>::put(something);
			log::info!("|||||||||||||||||||||| called by {:?}", who);
			print("After storing my_val");

			if_std! {
                println!("Hello native world!");
                println!("My value is: {:#?}", something);
                println!("The caller account is: {:#?}", who);
            }

			Self::deposit_event(Event::SomethingStored(something, who));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			log::info!("|||||||||||||||||||||| cause error");
			let _who = ensure_signed(origin)?;
			match <Something<T>>::get() {
				None => { 
					// print(Error::<T>::NoneValue);
					Err(Error::<T>::NoneValue)? 
				},
				Some(old) => {
					log::info!("|||||||||||||||||||||| 2 error");
					let new = old.checked_add(1).ok_or({
						// print(Error::<T>::StorageOverflow);
						Error::<T>::StorageOverflow
					})?;
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
