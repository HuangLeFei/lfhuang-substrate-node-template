#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer};
use sp_core::crypto::KeyTypeId;

/// 4个元素内容
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"hlf!");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_runtime::app_crypto::{app_crypto, sr25519};
	app_crypto!(sr25519, KEY_TYPE);
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		/// 链外工作者的标识符类型
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// The pallet's runtime storage items.
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	pub type Something<T> = StorageMap<_, Blake2_128Concat, u64, u64, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u64, u64),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		OffchainSignedTxError,
		NoLocalAcctForSigning,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// 在此函数中长时间的执行需要链下执行的功能。在区块导入Imported的时候调用。
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!(target: "lfhuang-ocw-signed-01", "before offchain_worker set storage: {:?}", block_number);
			let result = Self::block_number_send_signed(block_number);
			log::info!(target: "lfhuang-ocw-signed-02", "after offchain_worker set storage: {:?}", block_number);

			if let Err(e) = result {
				log::error!(target:"lfhuang-ocw-signed-03", "offchain_worker error: {:?}", e);
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn do_something_signed(
			origin: OriginFor<T>,
			number: u64,
		) -> DispatchResultWithPostInfo {
			// Retrieve sender of the transaction.
			let who = ensure_signed(origin)?;
			log::info!("🥸 🥸 🥸 ######################## signer：{:?}", who);

			let mut _number: u64 = 0;
			if number > 0 {
				_number = number;
			}
			log::info!(target:"ocw", "+++++++++++++++++++ offchain_worker set storage: {:?}, _number: {:?}", number, _number);

			Something::<T>::insert(&number, _number);

			Self::deposit_event(Event::SomethingStored(number, _number));
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn block_number_send_signed(block_number: T::BlockNumber) -> Result<(), Error<T>> {
			let signer = Signer::<T, T::AuthorityId>::all_accounts();
			log::info!(target:"lfhuang-ocw-signed-04", "+++++++++++++++++++, can sign: {:?}", signer.can_sign());
			if !signer.can_sign() {
				return Err(<Error<T>>::OffchainSignedTxError);
			}
			let number: u64 = block_number.try_into().unwrap_or(0);

			let result =
				signer.send_signed_transaction(|_account| Call::do_something_signed { number });

			for (acc, res) in &result {
				log::info!(target:"lfhuang-ocw-signed-05","------------ acc: [{:?}]", acc.id);
				if res.is_err() {
					return Err(<Error<T>>::OffchainSignedTxError);
				}
			}
			Ok(())
		}
	}
}
