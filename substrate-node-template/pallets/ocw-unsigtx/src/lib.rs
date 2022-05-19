#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use frame_system::offchain::{SendTransactionTypes, SubmitTransaction};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + SendTransactionTypes<Call<Self>> {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	pub type SomeInfo<T: Config> = StorageMap<_, Blake2_128Concat, u64, u64, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		UnsignedPutSetSomeInfo(u64, u64),
	}

	#[pallet::error]
	pub enum Error<T> {
		OffchainUnsignedTxError,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			let number: u64 = block_number.try_into().unwrap_or(0);
			let call = Call::submit_something_unsigned { number };

			if let Err(e) =
				SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
					.map_err(|_| <Error<T>>::OffchainUnsignedTxError)
			{
				log::error!(target:"ocw", "offchain_worker submit unsigned tx error: {:?}", e);
			} else {
				log::info!(target:"ocw", "offchain_worker submit unsigned tx success");
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn submit_something_unsigned(
			origin: OriginFor<T>,
			number: u64,
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?;

			let mut cnt: u64 = 0;
			if number > 0 {
				cnt = number;
			}

			log::info!(target:"ocw", "unsigned +++++++++++++++++++ offchain_worker set storage: {:?}, cnt: {:?}", number, cnt);
			SomeInfo::<T>::insert(&number, cnt);

			Self::deposit_event(Event::UnsignedPutSetSomeInfo(number, cnt));

			Ok(().into())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			if let Call::submit_something_unsigned { number: _ } = call {
				ValidTransaction::with_tag_prefix("OcwUnsigtx")
					.priority(TransactionPriority::max_value())
					.longevity(5)
					.propagate(false)
					.build()
			} else {
				InvalidTransaction::Call.into()
			}
		}
	}
}
