
//! Autogenerated weights for lfhuang_benchmarking_pallet_template
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-03, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: ``
//! HOSTNAME: `lfhuang.local`, CPU: `<UNKNOWN>`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// lfhuang_benchmarking_pallet_template
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// pallets/lfhuang-benchmarking-pallet/src/weights.rs
// --template
// ./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for lfhuang_benchmarking_pallet_template.
pub trait WeightInfo {
	fn do_something(s: u32, ) -> Weight;
}

/// Weights for lfhuang_benchmarking_pallet_template using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: LfhuangBenchmarkingPallet UserStorage (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn do_something(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  ``
		//  Estimated: ``
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(32_137_829,0 )
			// Standard Error: 13_838
			.saturating_add(Weight::from_parts(22_949, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: LfhuangBenchmarkingPallet UserStorage (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn do_something(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  ``
		//  Estimated: ``
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(32_137_829, 0)
			// Standard Error: 13_838
			.saturating_add(Weight::from_parts(22_949, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}