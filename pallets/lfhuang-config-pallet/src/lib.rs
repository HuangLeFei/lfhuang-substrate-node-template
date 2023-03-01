#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// 3. Runtime Configuration Trait
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// 4. Runtime Storage
	// 用storageMap存储ID账户信息，（key， value）分别对应的是Id和余额.
	#[pallet::storage]
	#[pallet::getter(fn account_info)]
	pub type AccountInfo<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>;

	// 5. Runtime Events
	// Can stringify event types to metadata.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetAccountInfo(u32, u128),
	}

	// 8. Runtime Errors
	#[pallet::error]
	pub enum Error<T> {
		// 相同账户的属性Id唯一
		SetAccountInfoDuplicate,
	}

	// 7. Extrinsics
	// Functions that are callable from outside the runtime.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_account_info(
			origin: OriginFor<T>,
			account_id: u32,
			account_balance: u128,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?; //检查是否调用者权限

			if AccountInfo::<T>::contains_key(account_id) {
				return Err(Error::<T>::SetAccountInfoDuplicate.into())
			}

			AccountInfo::<T>::insert(&account_id, &account_balance);
			Self::deposit_event(Event::SetAccountInfo(account_id, account_balance));

			Ok(().into())
		}
	}
}
