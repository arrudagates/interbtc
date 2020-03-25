/// Tests for BTC-Relay

use crate::{Module, Trait, Error, Event};
use sp_core::{U256, H256};
use frame_support::{impl_outer_origin, impl_outer_event, assert_ok, assert_err, parameter_types, weights::Weight};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
};
use bitcoin::types::*;

impl_outer_origin! {
	pub enum Origin for Test {}
}

mod test_events {
    pub use crate::Event;
}

impl_outer_event! {
    pub enum TestEvent for Test {
        test_events,
    }
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}
impl system::Trait for Test {
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
}

impl Trait for Test {
	type Event = TestEvent;
}

pub type System = system::Module<Test>;
pub type BTCRelay = Module<Test>;

pub struct ExtBuilder;

impl ExtBuilder {
    pub fn build() -> sp_io::TestExternalities {
        let mut storage = system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        sp_io::TestExternalities::from(storage)
    }
}


// fn ExtBuilder::build() -> sp_io::TestExternalities {
// 	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
// }


/// Initialize Function
#[test]
fn initialize_once_suceeds() {
    ExtBuilder::build().execute_with(|| {
        let block_height: u32 = 0;
        let block_header = vec![0u8; 80];
        let block_header_hash = H256Le::zero();
        assert_ok!(BTCRelay::initialize(Origin::signed(3), block_header, block_height));
       
        let init_event = TestEvent::test_events(
            Event::Initialized(block_height, block_header_hash),
        );
        assert!(System::events().iter().any(|a| a.event == init_event));
    })
}

#[test]
fn initialize_twice_fails() {
    ExtBuilder::build().execute_with(|| {
        let block_height: u32 = 0;
        let block_header = vec![0u8; 80];
        let block_header_hash = H256Le::zero();
        assert_ok!(BTCRelay::initialize(Origin::signed(3), block_header, block_height));

        let block_height_2: u32 = 0;
        let block_header_2 = vec![1u8; 80];
        assert_err!(BTCRelay::initialize(Origin::signed(3), block_header_2, block_height_2), Error::<Test>::AlreadyInitialized);
    })
}

/// StoreBlockHeader Function
#[test]
fn store_fork_once_suceeds() {
    ExtBuilder::build().execute_with(|| {
        let block_height: u32 = 1;
        let block_header = vec![1u8; 80];
        let block_header_hash = H256Le::zero();
        let chain_id: u32 = 2;
        assert_ok!(BTCRelay::store_block_header(Origin::signed(3), block_header));
       
        let store_event = TestEvent::test_events(
            Event::StoreForkHeader(chain_id, block_height, block_header_hash),
        );
        assert!(System::events().iter().any(|a| a.event == store_event));
    })
}


fn sample_genesis_header() -> String {
    "01000000a7c3299ed2475e1d6ea5ed18d5bfe243224add249cce99c5c67cc9fb00000000601c73862a0a7238e376f497783c8ecca2cf61a4f002ec8898024230787f399cb575d949ffff001d3a5de07f".to_string()
}

fn sample_genesis_height() -> u32 {
    10_000
}

fn sample_first_header() -> String {
    "01000000cb60e68ead74025dcfd4bf4673f3f71b1e678be9c6e6585f4544c79900000000c7f42be7f83eddf2005272412b01204352a5fddbca81942c115468c3c4ec2fff827ad949ffff001d21e05e45".to_string()
}

#[test]
fn test_verify_block_header_succeeds() {
    let genesis_header = bitcoin_spv::utils::deserialize_hex(&sample_first_header()[..]).unwrap();
    let genesis_height = sample_genesis_height();
    assert_ok!(BTCRelay::initialize(Origin::signed(3), genesis_header, genesis_height));
}




