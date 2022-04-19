#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports and Dependencies
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, transactional};
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
	#[pallet::storage]
	#[pallet::getter(fn my_param)]
	pub type Param<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub type SetFlag<T: Config> = StorageValue<_, bool, ValueQuery>;

	// 5. Runtime Events
	// Can stringify event types to metadata.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetParam(u32),
	}

	// 6. Hooks
	// 添加hooks函数
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			log::info!(target: "use-hooks", "++++++++++++ on_initialize, block number is {:?}", n);
			0
		}

		fn on_finalize(n: BlockNumberFor<T>) {
			log::info!(target: "use-hooks", "------------ on_finalize, block number is {:?}", n);
		}
	}

	// 8. Runtime Errors
	#[pallet::error]
	pub enum Error<T> {
		// 参数必须大于100
		ParamInvalid,
	}

	// 7. Extrinsics
	// Functions that are callable from outside the runtime.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[transactional]
		#[pallet::weight(0)]
		pub fn set_param_bigger_than_100(origin: OriginFor<T>, param: u32) -> DispatchResult {
			//1、判断调用者权限
			ensure_signed(origin)?;

			//2、开始业务逻辑
			//2.1、将标志位设置为true
			SetFlag::<T>::put(true);

			//2.2、如果参数大于100,则写入到storage praram中
			if param <= 100u32 {
				return Err(Error::<T>::ParamInvalid.into())
			}
			Param::<T>::put(param);

			//3、发出事件
			Self::deposit_event(Event::SetParam(param));
			log::info!(target: "use-hooks", "set param bigger then 100");

			Ok(().into())
		}
	}
}
