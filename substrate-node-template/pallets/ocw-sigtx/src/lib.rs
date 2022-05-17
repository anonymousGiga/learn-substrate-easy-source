#![cfg_attr(not(feature = "std"), no_std)]

use sp_core::crypto::KeyTypeId;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_runtime::app_crypto::{app_crypto, sr25519};
	app_crypto!(sr25519, KEY_TYPE);
}

pub type AuthorityId = crypto::Public;

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use frame_system::offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer,
	};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	pub type SomeInfo<T: Config> = StorageMap<_, Blake2_128Concat, u64, u64, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetSomeInfo(u64, u64),
	}

	#[pallet::error]
	pub enum Error<T> {
		OffchainSignedTxError,
		NoLocalAcctForSigning,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!(target: "ocw", "before offchain_worker set storage: {:?}", block_number);
			let result = Self::offchain_signed_tx(block_number);
			log::info!(target: "ocw", "after offchain_worker set storage: {:?}", block_number);

			if let Err(e) = result {
				log::error!(target:"ocw", "offchain_worker error: {:?}", e);
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn submit_something_signed(
			origin: OriginFor<T>,
			number: u64,
		) -> DispatchResultWithPostInfo {
			log::info!(target:"ocw", "11111 +++++++++++++++++++ ");
			ensure_signed(origin)?;

			let mut cnt: u64 = 0;
			if number > 0 {
				cnt = number;
			}

			log::info!(target:"ocw", "+++++++++++++++++++ offchain_worker set storage: {:?}, cnt: {:?}", number, cnt);
			SomeInfo::<T>::insert(&number, cnt);

			Self::deposit_event(Event::SetSomeInfo(number, cnt));
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn offchain_signed_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {
			let signer = Signer::<T, T::AuthorityId>::any_account();
			log::info!(target:"ocw", "+++++++++++++++++++, can sign: {:?}", signer.can_sign());

			let number: u64 = block_number.try_into().unwrap_or(0);

			let result =
				signer.send_signed_transaction(|_acct| Call::submit_something_signed { number });

			if let Some((_acc, res)) = result {
				if res.is_err() {
					return Err(<Error<T>>::OffchainSignedTxError)
				}
				Ok(())
			} else {
				Err(<Error<T>>::NoLocalAcctForSigning)
			}
		}
	}
}
