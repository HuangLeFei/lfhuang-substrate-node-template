#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, HasCompact};
use frame_support::traits::Currency;
use scale_info::TypeInfo;
use sp_core::crypto::UncheckedFrom;
use sp_runtime::traits::StaticLookup;
use sp_std::{fmt::Debug, prelude::*};

type BalanceOf<T> = <<T as pallet_contracts::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// 【1】pallet_contracts 功能用途：为运行时提供部署和执行 WebAssembly 智能合约的功能。
	#[pallet::config]
	pub trait Config: pallet_contracts::Config + frame_system::Config {}

	// 【2】call调度函数
	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T::AccountId: UncheckedFrom<T::Hash>,
		T::AccountId: AsRef<[u8]>,
		<BalanceOf<T> as HasCompact>::Type: Clone + Eq + PartialEq + Debug + TypeInfo + Encode,
	{
		#[pallet::weight(0)]
		pub fn sudo_call(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] value: BalanceOf<T>,
			gas_limit: Weight,
			storage_deposit_limit: Option<<BalanceOf<T> as codec::HasCompact>::Type>,
			data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			//添加下面这行，用于判断是否是root权限
			ensure_root(origin.clone())?;

			//直接调用pallet-contracts的call函数
			pallet_contracts::Pallet::<T>::call(
				origin,
				dest,
				value,
				gas_limit,
				storage_deposit_limit,
				data,
			)
		}
	}
}
