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
		AppCrypto, CreateSignedTransaction, SendUnsignedTransaction, SignedPayload, Signer,
		SigningTypes,
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
		UnsignedPutSetSomeInfo(u64, u64),
	}

	#[pallet::error]
	pub enum Error<T> {
		OffchainUnsignedTxError,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct SomethingPayload<Public, BlockNumber> {
		block_number: BlockNumber,
		something: u64,
		public: Public,
	}

	impl<T: SigningTypes> SignedPayload<T> for SomethingPayload<T::Public, T::BlockNumber> {
		fn public(&self) -> T::Public {
			self.public.clone()
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			let number: u64 = block_number.try_into().unwrap_or(0);

			let _ = Signer::<T, T::AuthorityId>::any_account().send_unsigned_transaction(
				|account| SomethingPayload {
					block_number,
					something: number,
					public: account.public.clone(),
				},
				|payload, signature| Call::submit_something_unsigned_with_signed_payload {
					something_payload: payload,
					signature,
				},
			);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn submit_something_unsigned_with_signed_payload(
			origin: OriginFor<T>,
			something_payload: SomethingPayload<T::Public, T::BlockNumber>,
			_signature: T::Signature,
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?;

			let mut cnt: u64 = 0;
			let number: u64 = something_payload.block_number.try_into().unwrap_or(0);
			if number > 0 {
				cnt = number;
			}

			log::info!(target:"ocw", "unsigned with signed payload +++++++++++number: {:?}, cnt: {:?}", number, cnt);
			SomeInfo::<T>::insert(&number, cnt);

			Self::deposit_event(Event::UnsignedPutSetSomeInfo(number, cnt));

			Ok(().into())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			if let Call::submit_something_unsigned_with_signed_payload {
				something_payload: ref payload,
				ref signature,
			} = call
			{
				let signature_valid =
					SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone());
				if !signature_valid {
					return InvalidTransaction::BadProof.into()
				}

				ValidTransaction::with_tag_prefix("OcwUnsigtxPayload")
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
