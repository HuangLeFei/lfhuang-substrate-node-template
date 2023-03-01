#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::print;
	use sp_runtime::traits::Printable;
	use sp_std::if_std;

	// Declare the pallet type
	// This is a placeholder to implement traits and methods.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Add the runtime configuration trait
	// All types and constants go here.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// Add runtime storage to declare storage items.
	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T: Config> = StorageValue<_, u32>;

	// Add runtime events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
	}

	impl<T: Config> Printable for Error<T> {
		fn print(&self) {
			match self {
				Error::NoneValue => "Invalid Value".print(),
				Error::StorageOverflow => "Value Exceeded and Overflowed".print(),
				_ => "Invalid Error Case".print(),
			}
		}
	}

	// Add hooks to define some logic that should be executed
	// in a specific context, for example on_initialize.
	//  #[pallet::hooks]
	//  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { ... }

	// Add functions that are callable from outside the runtime.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?; // 检查是否已签名
			print("🥸 🥸 🥸 开始设置值..."); // Debug方式三 print，默认是debug级别，所以启动时候把环境变量调整到debug启动

			<Something<T>>::put(something);

			print("🥸 🥸 🥸 存储值完成...");

			// Debug方式一 log
			log::info!(
				"🥸 🥸 🥸 ######################## called by something:{:?} and signer：{:?}",
				something,
				who
			);

			// Debug方式四 if_std!宏
			if_std! {
				println!("😂 😂 😂 ################## Hello native world! something value: {}",something);
				println!("😂 😂 😂 ################## 调用者的交易账户是: {:#?}", who);
			}

			Self::deposit_event(Event::SomethingStored(something, who));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			// 检查是否已签名
			let _who = ensure_signed(origin)?;

			print("🥸 🥸 🥸 ########### 错误原因... #############");
			match <Something<T>>::get() {
				None => {
					print(Error::<T>::NoneValue); // Debug方式二 Printable，默认是debug级别，所以启动时候把环境变量调整到debug启动
					Err(Error::<T>::NoneValue)?
				},
				Some(old) => {
					let new = old.checked_add(1).ok_or({
						print(Error::<T>::StorageOverflow);
						Error::<T>::StorageOverflow
					})?;
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
