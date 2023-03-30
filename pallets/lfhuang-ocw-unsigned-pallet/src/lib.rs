#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use frame_system::offchain::{SendTransactionTypes, SubmitTransaction};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// 在Config需要继承trait SendTransactionTypes<Call>才能在ocw提交未签名交易。
	#[pallet::config]
	pub trait Config: SendTransactionTypes<Call<Self>> + frame_system::Config {
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
		UnsignedSomethingStored(u64, u64),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		OffchainUnsignedTxError,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// 在此函数中长时间的执行需要链下执行的功能。在区块导入Imported的时候调用。
		fn offchain_worker(block_number: T::BlockNumber) {
			let number: u64 = block_number.try_into().unwrap_or(0);
			log::info!(target: "lfhuang-ocw-unsigned-01", "before offchain_worker ocw-unsigned set storage: {:?}", block_number);
			//下面为具体的调用未签名交易的方式
			let call = Call::<T>::do_something_unsigned { number };

			if let Err(e) =
				SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
					.map_err(|_| <Error<T>>::OffchainUnsignedTxError)
			{
				log::error!(target:"lfhuang-ocw-unsigned-02", "offchain_worker submit unsigned tx error: {:?}", e);
			} else {
				log::info!(target:"lfhuang-ocw-unsigned-03", "offchain_worker submit unsigned tx success");
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// 实现具体的未签名调度函数
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn do_something_unsigned(
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
			log::info!(target:"ocw", "########################### offchain_worker ocw-unsigned set storage: {:?}, _number: {:?}", number, _number);

			Something::<T>::insert(&number, _number);

			Self::deposit_event(Event::UnsignedSomethingStored(number, _number));
			Ok(().into())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		// 实现未签名交易验证的trait
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			if let Call::do_something_unsigned { number: _ } = call {
				const PREFIX: &str = "lfhuangUnsigned";
				/// 使用特定的标签前缀启动构建器对象。
				ValidTransaction::with_tag_prefix(PREFIX)
					.priority(TransactionPriority::max_value()) //优先级确定满足所有依赖项（必需标签）的两个事务的顺序。
					.longevity(2) //交易寿命周期区块数
					.propagate(false) //事务传播：false交易仍将被考虑包含在当前节点上创建的块中，但永远不会发送给其他节点。
					.build()
			} else {
				InvalidTransaction::Call.into()
			}
		}
	}
}
