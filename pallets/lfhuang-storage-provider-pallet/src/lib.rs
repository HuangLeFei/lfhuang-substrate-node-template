#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use traits::StorageInterface; //实现自定义的traits接口特征

pub mod traits;

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

	#[pallet::config]
	pub trait Config: frame_system::Config {
		// config中定义Event和Value关联类型
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
	}

	#[pallet::storage]
	pub type MyStorageValue<T: Config> = StorageValue<_, T::Value, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		FunctionCall(u32),
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn my_function(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			let value = MyStorageValue::<T>::get();
			log::info!(target: "storage provider", "😀😀😀 ####lfhuang-storage-provider-pallet-template#### my function! Value get from storage is: {:?}", value);
			Self::deposit_event(Event::FunctionCall(value.into()));

			Ok(().into())
		}
	}
}

impl<T: Config> StorageInterface for Pallet<T> {
	type Value = T::Value;

	fn get_param() -> Self::Value {
		MyStorageValue::<T>::get()
	}

	fn set_param(v: Self::Value) {
		MyStorageValue::<T>::put(v);
	}
}
