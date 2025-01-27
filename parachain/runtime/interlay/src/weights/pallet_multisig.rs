
//! Autogenerated weights for pallet_multisig
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-05, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `dev-benchmark`, CPU: `Intel(R) Xeon(R) CPU @ 2.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("interlay-dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// interlay-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 50
// --repeat
// 10
// --output
// ../tmp-weight/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_multisig using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {

	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1	(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 40_254_000 picoseconds.
		Weight::from_parts(45_954_521, 0)
			// Standard Error: 78
			.saturating_add(Weight::from_parts(1_088, 0).saturating_mul(z.into()))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create	(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `679 + s * (3 ±0)`
		//  Estimated: `10391`
		// Minimum execution time: 100_166_000 picoseconds.
		Weight::from_parts(78_077_241, 10391)
			// Standard Error: 12_891
			.saturating_add(Weight::from_parts(500_683, 0).saturating_mul(s.into()))
			// Standard Error: 126
			.saturating_add(Weight::from_parts(2_720, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve	(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `210`
		//  Estimated: `6811`
		// Minimum execution time: 67_053_000 picoseconds.
		Weight::from_parts(49_470_953, 6811)
			// Standard Error: 10_765
			.saturating_add(Weight::from_parts(372_231, 0).saturating_mul(s.into()))
			// Standard Error: 105
			.saturating_add(Weight::from_parts(2_250, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete	(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `820 + s * (35 ±0)`
		//  Estimated: `10391`
		// Minimum execution time: 98_109_000 picoseconds.
		Weight::from_parts(88_799_005, 10391)
			// Standard Error: 15_677
			.saturating_add(Weight::from_parts(531_558, 0).saturating_mul(s.into()))
			// Standard Error: 153
			.saturating_add(Weight::from_parts(2_801, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create	(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `681 + s * (3 ±0)`
		//  Estimated: `10391`
		// Minimum execution time: 74_815_000 picoseconds.
		Weight::from_parts(82_028_501, 10391)
			// Standard Error: 14_163
			.saturating_add(Weight::from_parts(476_567, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve	(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `210`
		//  Estimated: `6811`
		// Minimum execution time: 44_360_000 picoseconds.
		Weight::from_parts(48_304_264, 6811)
			// Standard Error: 11_358
			.saturating_add(Weight::from_parts(370_201, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi	(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `887 + s * (3 ±0)`
		//  Estimated: `10391`
		// Minimum execution time: 70_033_000 picoseconds.
		Weight::from_parts(78_599_449, 10391)
			// Standard Error: 16_395
			.saturating_add(Weight::from_parts(444_403, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}