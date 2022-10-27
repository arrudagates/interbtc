//! Autogenerated weights for issue
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-08, STEPS: `100`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/interbtc-standalone
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// issue
// --extrinsic
// *
// --steps
// 100
// --repeat
// 10
// --output
// crates/issue/src/default_weights.rs
// --template
// .deploy/weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for issue.
pub trait WeightInfo {
	fn request_issue() -> Weight;
	fn execute_issue() -> Weight;
	fn cancel_issue() -> Weight;
	fn set_issue_period() -> Weight;
}

/// Weights for issue using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: BTCRelay StartBlockHeight (r:1 w:0)
	// Storage: BTCRelay StableBitcoinConfirmations (r:1 w:0)
	// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: Fee IssueGriefingCollateral (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Issue IssueBtcDustValue (r:1 w:0)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: Staking Nonce (r:1 w:0)
	// Storage: Staking TotalCurrentStake (r:1 w:0)
	// Storage: Fee IssueFee (r:1 w:0)
	// Storage: Security Nonce (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: Security ActiveBlockCount (r:1 w:0)
	// Storage: Issue IssuePeriod (r:1 w:0)
	// Storage: Issue IssueRequests (r:0 w:1)
	fn request_issue() -> Weight {
		Weight::from_ref_time(500_076_000 as u64)
			.saturating_add(T::DbWeight::get().reads(17 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Issue IssueRequests (r:1 w:1)
	// Storage: Issue IssuePeriod (r:1 w:0)
	// Storage: Security ActiveBlockCount (r:1 w:0)
	// Storage: BTCRelay DisableInclusionCheck (r:1 w:0)
	// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	// Storage: BTCRelay Chains (r:1 w:0)
	// Storage: BTCRelay BlockHeaders (r:1 w:0)
	// Storage: BTCRelay StableBitcoinConfirmations (r:1 w:0)
	// Storage: BTCRelay StableParachainConfirmations (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	// Storage: Rewards Stake (r:1 w:1)
	// Storage: Rewards TotalStake (r:1 w:1)
	// Storage: Rewards RewardTally (r:1 w:1)
	// Storage: Rewards RewardPerToken (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Rewards TotalRewards (r:1 w:1)
	fn execute_issue() -> Weight {
		Weight::from_ref_time(203_467_000 as u64)
			.saturating_add(T::DbWeight::get().reads(19 as u64))
			.saturating_add(T::DbWeight::get().writes(10 as u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Issue IssueRequests (r:1 w:1)
	// Storage: Issue IssuePeriod (r:1 w:0)
	// Storage: Security ActiveBlockCount (r:1 w:0)
	// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	fn cancel_issue() -> Weight {
		Weight::from_ref_time(95_611_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Issue IssuePeriod (r:0 w:1)
	fn set_issue_period() -> Weight {
		Weight::from_ref_time(3_071_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: BTCRelay StartBlockHeight (r:1 w:0)
	// Storage: BTCRelay StableBitcoinConfirmations (r:1 w:0)
	// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: Fee IssueGriefingCollateral (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Issue IssueBtcDustValue (r:1 w:0)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: Staking Nonce (r:1 w:0)
	// Storage: Staking TotalCurrentStake (r:1 w:0)
	// Storage: Fee IssueFee (r:1 w:0)
	// Storage: Security Nonce (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: Security ActiveBlockCount (r:1 w:0)
	// Storage: Issue IssuePeriod (r:1 w:0)
	// Storage: Issue IssueRequests (r:0 w:1)
	fn request_issue() -> Weight {
		Weight::from_ref_time(500_076_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(17 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Issue IssueRequests (r:1 w:1)
	// Storage: Issue IssuePeriod (r:1 w:0)
	// Storage: Security ActiveBlockCount (r:1 w:0)
	// Storage: BTCRelay DisableInclusionCheck (r:1 w:0)
	// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	// Storage: BTCRelay Chains (r:1 w:0)
	// Storage: BTCRelay BlockHeaders (r:1 w:0)
	// Storage: BTCRelay StableBitcoinConfirmations (r:1 w:0)
	// Storage: BTCRelay StableParachainConfirmations (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	// Storage: Rewards Stake (r:1 w:1)
	// Storage: Rewards TotalStake (r:1 w:1)
	// Storage: Rewards RewardTally (r:1 w:1)
	// Storage: Rewards RewardPerToken (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Rewards TotalRewards (r:1 w:1)
	fn execute_issue() -> Weight {
		Weight::from_ref_time(203_467_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(19 as u64))
			.saturating_add(RocksDbWeight::get().writes(10 as u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Issue IssueRequests (r:1 w:1)
	// Storage: Issue IssuePeriod (r:1 w:0)
	// Storage: Security ActiveBlockCount (r:1 w:0)
	// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	fn cancel_issue() -> Weight {
		Weight::from_ref_time(95_611_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Issue IssuePeriod (r:0 w:1)
	fn set_issue_period() -> Weight {
		Weight::from_ref_time(3_071_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}

