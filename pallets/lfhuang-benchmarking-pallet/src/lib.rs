#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo; //引入导出的weights依赖

#[frame_support::pallet]
pub mod pallet {
	use crate::WeightInfo; // add this line
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
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type UserType: Member
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Codec
			+ Copy
			+ Debug
			+ Default
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize;
		type WeightInfo: WeightInfo; // add this line
	}

	#[pallet::storage]
	#[pallet::getter(fn user_storage)]
	pub type UserStorage<T: Config> = StorageValue<_, T::UserType, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		UserStored(T::UserType),
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub user: T::UserType,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { user: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			UserStorage::<T>::put(self.user);
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		#[pallet::weight(<T as Config>::WeightInfo::do_something(22))]
		pub fn do_user_info(origin: OriginFor<T>, user: T::UserType) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// root账户进行签名  需要使用sudo权限执行
			// let who = ensure_root(origin)?;
			log::info!(
				"🥸 🥸 🥸 ######################## Benchmarking called by signer：{:?}",
				who
			);
			<UserStorage<T>>::put(user);

			Self::deposit_event(Event::UserStored(user));
			Ok(())
		}
	}
}
