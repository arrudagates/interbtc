
//! Autogenerated weights for oracle
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-14, STEPS: `100`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `sander-dell`, CPU: `11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kintsugi-dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// kintsugi-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 100
// --repeat
// 10
// --output
// /tmp/weights/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for oracle using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> oracle::WeightInfo for WeightInfo<T> {
	/// Storage: Oracle RawValuesUpdated (r:1001 w:1000)
	/// Proof: Oracle RawValuesUpdated (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Oracle RawValues (r:2000 w:0)
	/// Proof: Oracle RawValues (max_values: None, max_size: Some(100), added: 2575, mode: MaxEncodedLen)
	/// Storage: Oracle MaxDelay (r:1 w:0)
	/// Proof: Oracle MaxDelay (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: VaultRewards TotalStake (r:1000 w:0)
	/// Proof: VaultRewards TotalStake (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Security ParachainStatus (r:1 w:1)
	/// Proof: Security ParachainStatus (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: Security Errors (r:1 w:1)
	/// Proof: Security Errors (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: Oracle Aggregate (r:0 w:1000)
	/// Proof: Oracle Aggregate (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: Oracle ValidUntil (r:0 w:1000)
	/// Proof: Oracle ValidUntil (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	/// The range of component `u` is `[1, 1000]`.
	fn on_initialize(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1536 + u * (127 ±0)`
		//  Estimated: `4503 + u * (10172 ±0)`
		// Minimum execution time: 120_736_000 picoseconds.
		Weight::from_parts(123_175_000, 4503)
			// Standard Error: 68_596
			.saturating_add(Weight::from_parts(28_172_810, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 10172).saturating_mul(u.into()))
	}
	/// Storage: Oracle AuthorizedOracles (r:1 w:0)
	/// Proof: Oracle AuthorizedOracles (max_values: None, max_size: Some(305), added: 2780, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Oracle RawValuesUpdated (r:0 w:1000)
	/// Proof: Oracle RawValuesUpdated (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	/// Storage: Oracle RawValues (r:0 w:1000)
	/// Proof: Oracle RawValues (max_values: None, max_size: Some(100), added: 2575, mode: MaxEncodedLen)
	/// The range of component `u` is `[1, 1000]`.
	fn feed_values(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1316`
		//  Estimated: `3283`
		// Minimum execution time: 50_489_000 picoseconds.
		Weight::from_parts(52_591_000, 3283)
			// Standard Error: 15_767
			.saturating_add(Weight::from_parts(4_229_218, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(u.into())))
	}
	/// Storage: Oracle AuthorizedOracles (r:0 w:1)
	/// Proof: Oracle AuthorizedOracles (max_values: None, max_size: Some(305), added: 2780, mode: MaxEncodedLen)
	fn insert_authorized_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1021`
		//  Estimated: `0`
		// Minimum execution time: 21_919_000 picoseconds.
		Weight::from_parts(22_720_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Oracle AuthorizedOracles (r:0 w:1)
	/// Proof: Oracle AuthorizedOracles (max_values: None, max_size: Some(305), added: 2780, mode: MaxEncodedLen)
	fn remove_authorized_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1021`
		//  Estimated: `0`
		// Minimum execution time: 21_188_000 picoseconds.
		Weight::from_parts(21_463_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}