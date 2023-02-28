#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::Codec;
	use frame_support::{
		pallet_prelude::*, sp_runtime::traits::AtLeast32BitUnsigned, sp_std::fmt::Debug,
	};
	use frame_system::pallet_prelude::*;
	use lfhuang_storage_provider_pallet_template::traits::StorageInterface; //实现自定义的traits接口特征，需要把另一个pallet的依赖也加载进来。

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// 3. Runtime Configuration Trait
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// config中定义Event、Value和MyStorage关联类型
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Value: Member
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
		// 定义了MyStorage的类型，要求实现trait StorageInterface
		type MyStorage: StorageInterface<Value = Self::Value>;
	}

	// 5. Runtime Events
	// Can stringify event types to metadata.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		StoreEvent(u32),
	}

	// 7. Extrinsics
	// Functions that are callable from outside the runtime.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn storage_value(origin: OriginFor<T>, value: T::Value) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			T::MyStorage::set_param(value);

			// 使用trait中的 StorageInterface的函数
			let v = T::MyStorage::get_param();
			log::info!(target: "other-pallet", "😀😀😀 ####lfhuang-use-storage-pallet-template#### Value get from storage is: {:?}", v);

			Self::deposit_event(Event::StoreEvent(v.into()));

			Ok(().into())
		}
	}
}
