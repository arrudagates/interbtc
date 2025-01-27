
//! Autogenerated weights for dex_stable
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-10, STEPS: `10`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: ``, CPU: `Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kintsugi-dev"), DB CACHE: 1024

// Executed Command:
// ./target/debug/interbtc-parachain
// benchmark
// pallet
// --pallet
// dex-stable
// --extrinsic
// *
// --chain
// kintsugi-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 10
// --repeat
// 1
// --output
// parachain/runtime/kintsugi/src/weights
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for dex_stable using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> dex_stable::WeightInfo for WeightInfo<T> {

	/// Storage: DexStable NextPoolId (r:1 w:1)
	/// Proof: DexStable NextPoolId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: DexStable LpCurrencies (r:1 w:1)
	/// Proof: DexStable LpCurrencies (max_values: None, max_size: Some(31), added: 2506, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	/// The range of component `s` is `[0, 50]`.
	fn create_base_pool	(b: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `175`
		//  Estimated: `4281`
		// Minimum execution time: 923_222_000 picoseconds.
		Weight::from_parts(930_939_034, 4281)
			// Standard Error: 317_902
			.saturating_add(Weight::from_parts(154_311, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: DexStable LpCurrencies (r:2 w:1)
	/// Proof: DexStable LpCurrencies (max_values: None, max_size: Some(31), added: 2506, mode: MaxEncodedLen)
	/// Storage: DexStable NextPoolId (r:1 w:1)
	/// Proof: DexStable NextPoolId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DexStable Pools (r:2 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// The range of component `m` is `[2, 10]`.
	/// The range of component `s` is `[0, 50]`.
	fn create_meta_pool	(m: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1355`
		//  Estimated: `7572`
		// Minimum execution time: 1_510_682_000 picoseconds.
		Weight::from_parts(1_509_960_386, 7572)
			// Standard Error: 602_158
			.saturating_add(Weight::from_parts(914_689, 0).saturating_mul(m.into()))
			// Standard Error: 103_829
			.saturating_add(Weight::from_parts(96_679, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:21 w:21)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	fn add_liquidity	(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `948 + b * (125 ±0)`
		//  Estimated: `4281 + b * (5180 ±0)`
		// Minimum execution time: 3_222_842_000 picoseconds.
		Weight::from_parts(1_446_309_446, 4281)
			// Standard Error: 885_938
			.saturating_add(Weight::from_parts(889_673_438, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn swap	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2108`
		//  Estimated: `11350`
		// Minimum execution time: 2_137_339_000 picoseconds.
		Weight::from_parts(2_137_339_000, 11350)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:21 w:21)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	fn remove_liquidity	(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1057 + b * (192 ±0)`
		//  Estimated: `4281 + b * (5180 ±0)`
		// Minimum execution time: 2_515_350_000 picoseconds.
		Weight::from_parts(1_306_368_241, 4281)
			// Standard Error: 2_505_739
			.saturating_add(Weight::from_parts(599_062_653, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn remove_liquidity_one_currency	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2207`
		//  Estimated: `8760`
		// Minimum execution time: 2_167_491_000 picoseconds.
		Weight::from_parts(2_167_491_000, 8760)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:21 w:21)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	fn remove_liquidity_imbalance	(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1099 + b * (192 ±0)`
		//  Estimated: `4281 + b * (5180 ±0)`
		// Minimum execution time: 2_586_449_000 picoseconds.
		Weight::from_parts(1_361_082_704, 4281)
			// Standard Error: 2_175_713
			.saturating_add(Weight::from_parts(608_779_034, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:41 w:41)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	/// The range of component `m` is `[2, 10]`.
	fn add_pool_and_base_pool_liquidity	(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1437 + b * (187 ±0) + m * (121 ±0)`
		//  Estimated: `7572 + b * (5180 ±0) + m * (5180 ±0)`
		// Minimum execution time: 11_589_457_000 picoseconds.
		Weight::from_parts(2_489_097_862, 7572)
			// Standard Error: 1_840_926
			.saturating_add(Weight::from_parts(730_145_710, 0).saturating_mul(b.into()))
			// Standard Error: 1_840_926
			.saturating_add(Weight::from_parts(896_050_781, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(m.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(m.into()))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:41 w:41)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 10]`.
	/// The range of component `m` is `[2, 10]`.
	fn remove_pool_and_base_pool_liquidity	(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1658 + b * (187 ±0) + m * (187 ±0)`
		//  Estimated: `7572 + b * (5180 ±0) + m * (5180 ±0)`
		// Minimum execution time: 9_593_347_000 picoseconds.
		Weight::from_parts(2_422_602_274, 7572)
			// Standard Error: 3_066_114
			.saturating_add(Weight::from_parts(599_022_504, 0).saturating_mul(b.into()))
			// Standard Error: 3_066_114
			.saturating_add(Weight::from_parts(598_545_261, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(5_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(m.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(m.into()))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:5 w:5)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn remove_pool_and_base_pool_liquidity_one_currency	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4012`
		//  Estimated: `13940`
		// Minimum execution time: 4_107_621_000 picoseconds.
		Weight::from_parts(4_107_621_000, 13940)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:15 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn swap_pool_from_base	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4279`
		//  Estimated: `39840`
		// Minimum execution time: 5_638_902_000 picoseconds.
		Weight::from_parts(5_638_902_000, 39840)
			.saturating_add(T::DbWeight::get().reads(20_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: DexStable Pools (r:2 w:2)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:6 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	fn swap_pool_to_base	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4042`
		//  Estimated: `16530`
		// Minimum execution time: 4_095_901_000 picoseconds.
		Weight::from_parts(4_095_901_000, 16530)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn swap_meta_pool_underlying	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2459`
		//  Estimated: `11350`
		// Minimum execution time: 2_196_520_000 picoseconds.
		Weight::from_parts(2_196_520_000, 11350)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	fn update_fee_receiver	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `874`
		//  Estimated: `4281`
		// Minimum execution time: 505_835_000 picoseconds.
		Weight::from_parts(505_835_000, 4281)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	fn set_swap_fee	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `874`
		//  Estimated: `4281`
		// Minimum execution time: 516_813_000 picoseconds.
		Weight::from_parts(516_813_000, 4281)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	fn set_admin_fee	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `874`
		//  Estimated: `4281`
		// Minimum execution time: 506_259_000 picoseconds.
		Weight::from_parts(506_259_000, 4281)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	fn ramp_a	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `968`
		//  Estimated: `4281`
		// Minimum execution time: 651_728_000 picoseconds.
		Weight::from_parts(651_728_000, 4281)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn stop_ramp_a	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `968`
		//  Estimated: `4281`
		// Minimum execution time: 643_370_000 picoseconds.
		Weight::from_parts(643_370_000, 4281)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DexStable Pools (r:1 w:1)
	/// Proof: DexStable Pools (max_values: None, max_size: Some(816), added: 3291, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:10 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	fn withdraw_admin_fee	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1843`
		//  Estimated: `26890`
		// Minimum execution time: 2_877_773_000 picoseconds.
		Weight::from_parts(2_877_773_000, 26890)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}