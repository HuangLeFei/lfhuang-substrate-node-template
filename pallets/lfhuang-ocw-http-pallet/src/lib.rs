#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use frame_support::inherent::Vec;
	use lite_json::json::JsonValue;
	use sp_runtime::offchain::{http, Duration};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored { something: u32, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_block_number: T::BlockNumber) {
			let price = Self::fetch_price().map_err(|_| "Failed to fetch price");

			log::info!(target:"offchain-ocw-http", "### 1 offchain-ocw-http get price: {:?}", price);

			if let Ok(data) = Self::fetch_price() {
				log::info!(target:"offchain-ocw-http", "### 2 offchain-ocw-http get price: {:?}", data);
			} else {
				log::info!(target:"offchain-ocw-http", "!!!offchain-ocw-http get price: failed ==================== ");
			}
		}
	}

	impl<T: Config> Pallet<T> {
		fn parse_price(price_str: &str) -> Option<u32> {
			// 解析JSON并打印生成的lite-JSON结构。
			let val = lite_json::parse_json(price_str);
			let price = match val.ok()? {
				JsonValue::Object(obj) => {
					let (_, v) =
						obj.into_iter().find(|(k, _)| k.iter().copied().eq("USD".chars()))?;
					match v {
						JsonValue::Number(number) => number,
						_ => return None,
					}
				},
				_ => return None,
			};

			let exp = price.fraction_length.saturating_sub(2);
			Some(price.integer as u32 * 100 + (price.fraction / 10_u64.pow(exp)) as u32)
		}

		fn fetch_price() -> Result<u32, http::Error> {
			// 设置截止日期并实例化 HTTP 请求
			// 创建 2 秒的截止时间。
			let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));

			//发起外部 HTTP GET 请求。
			let request = http::Request::get(
				"https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD",
			);

			let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;

			let response =
				pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
			// 检查响应状态代码。
			if response.code != 200 {
				log::warn!("Unexpected status code: {}", response.code);
				return Err(http::Error::Unknown);
			}

			// 读取body内容
			let body = response.body().collect::<Vec<u8>>();

			let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
				log::warn!("No UTF8 body");
				http::Error::Unknown
			})?;
			log::warn!("body_str: {} ", body_str);

			let price = match Self::parse_price(body_str) {
				Some(price) => Ok(price),
				None => {
					log::warn!("Unable to extract price from the response: {:?}", body_str);
					Err(http::Error::Unknown)
				},
			}?;

			log::warn!("Got price: {} cents", price);

			Ok(price)
		}
	}
}
