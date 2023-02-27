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

		// [1.0] 声明AccountIdType类型
		type AccountIdType: Member
			+ Parameter
			+ Copy
			+ Default
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize;

		// [1.1] 声明AccountBalanceType类型
		type AccountBalanceType: Parameter
			+ Member
			+ Default
			+ From<u128>
			+ Into<u128>
			+ Copy
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize;
	}

	// 4. Runtime Storage
	// 用storageMap存储ID账户信息，（key， value）分别对应的是Id和余额.
	// [2] 使用了
	#[pallet::storage]
	#[pallet::getter(fn account_info)]
	pub type AccountInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountIdType, T::AccountBalanceType, ValueQuery>;
	// StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>; // 实际开中格式类型u32,u128可能比较多,通过在Runtime中指定具体类型来开发

	// 5. Runtime Events
	// Can stringify event types to metadata.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// [3] 替换原来的事件类型参数u32和u128
		SetAccountInfo(T::AccountIdType, T::AccountBalanceType),
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
		// [4] 替换原来的事件类型参数u32和u128
		#[pallet::weight(0)]
		pub fn set_account_info(
			origin: OriginFor<T>,
			account_id: T::AccountIdType,
			account_balance: T::AccountBalanceType,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?; //检查是否调用者权限

			if AccountInfo::<T>::contains_key(account_id) {
				return Err(Error::<T>::SetAccountInfoDuplicate.into());
			}

			AccountInfo::<T>::insert(&account_id, &account_balance);
			Self::deposit_event(Event::SetAccountInfo(account_id, account_balance));

			Ok(().into())
		}
	}
}
