use crate as issue;
use crate::{ext, Config, Error};
use currency::Amount;
use frame_support::{
    assert_ok, parameter_types,
    traits::{ConstU32, Everything, GenesisBuild},
    PalletId,
};
use mocktopus::{macros::mockable, mocking::*};
use orml_traits::parameter_type_with_key;
pub use primitives::{CurrencyId, CurrencyId::Token, TokenSymbol::*};
use primitives::{VaultCurrencyPair, VaultId};
use sp_arithmetic::{FixedI128, FixedPointNumber, FixedU128};
use sp_core::H256;
use sp_runtime::{
    testing::{Header, TestXt},
    traits::{BlakeTwo256, Convert, IdentityLookup, One, Zero},
};

type TestExtrinsic = TestXt<RuntimeCall, ()>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},

        // Tokens & Balances
        Tokens: orml_tokens::{Pallet, Storage, Config<T>, Event<T>},

        CapacityRewards: reward::<Instance1>::{Pallet, Call, Storage, Event<T>},
        VaultRewards: reward::<Instance2>::{Pallet, Call, Storage, Event<T>},
        VaultStaking: staking::{Pallet, Storage, Event<T>},

        // Operational
        BTCRelay: btc_relay::{Pallet, Call, Config<T>, Storage, Event<T>},
        Security: security::{Pallet, Call, Storage, Event<T>},
        VaultRegistry: vault_registry::{Pallet, Call, Config<T>, Storage, Event<T>},
        Oracle: oracle::{Pallet, Call, Config<T>, Storage, Event<T>},
        Issue: issue::{Pallet, Call, Config<T>, Storage, Event<T>},
        Fee: fee::{Pallet, Call, Config<T>, Storage},
        Currency: currency::{Pallet},
        Nomination: nomination::{Pallet, Call, Config, Storage, Event<T>},
    }
);

pub type AccountId = u64;
pub type Balance = u128;
pub type RawAmount = i128;
pub type BlockNumber = u64;
pub type Moment = u64;
pub type Index = u64;
pub type SignedFixedPoint = FixedI128;
pub type SignedInner = i128;
pub type UnsignedFixedPoint = FixedU128;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub const DEFAULT_COLLATERAL_CURRENCY: CurrencyId = Token(DOT);
pub const DEFAULT_NATIVE_CURRENCY: CurrencyId = Token(INTR);
pub const DEFAULT_WRAPPED_CURRENCY: CurrencyId = Token(IBTC);

parameter_types! {
    pub const GetCollateralCurrencyId: CurrencyId = DEFAULT_COLLATERAL_CURRENCY;
    pub const GetNativeCurrencyId: CurrencyId = DEFAULT_NATIVE_CURRENCY;
    pub const GetWrappedCurrencyId: CurrencyId = DEFAULT_WRAPPED_CURRENCY;
    pub const MaxLocks: u32 = 50;
}

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
        Zero::zero()
    };
}

impl orml_tokens::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type Amount = RawAmount;
    type CurrencyId = CurrencyId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type CurrencyHooks = ();
    type MaxLocks = MaxLocks;
    type DustRemovalWhitelist = Everything;
    type MaxReserves = ConstU32<0>; // we don't use named reserves
    type ReserveIdentifier = (); // we don't use named reserves
}

type CapacityRewardsInstance = reward::Instance1;

impl reward::Config<CapacityRewardsInstance> for Test {
    type RuntimeEvent = RuntimeEvent;
    type SignedFixedPoint = SignedFixedPoint;
    type PoolId = ();
    type StakeId = CurrencyId;
    type CurrencyId = CurrencyId;
    type MaxRewardCurrencies = ConstU32<10>;
}

type VaultRewardsInstance = reward::Instance2;

impl reward::Config<VaultRewardsInstance> for Test {
    type RuntimeEvent = RuntimeEvent;
    type SignedFixedPoint = SignedFixedPoint;
    type PoolId = CurrencyId;
    type StakeId = VaultId<AccountId, CurrencyId>;
    type CurrencyId = CurrencyId;
    type MaxRewardCurrencies = ConstU32<10>;
}

impl staking::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type SignedFixedPoint = SignedFixedPoint;
    type SignedInner = SignedInner;
    type CurrencyId = CurrencyId;
    type GetNativeCurrencyId = GetNativeCurrencyId;
}

parameter_types! {
    pub const VaultPalletId: PalletId = PalletId(*b"mod/vreg");
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Test
where
    RuntimeCall: From<C>,
{
    type OverarchingCall = RuntimeCall;
    type Extrinsic = TestExtrinsic;
}

impl vault_registry::Config for Test {
    type PalletId = VaultPalletId;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type GetGriefingCollateralCurrencyId = GetNativeCurrencyId;
}

impl nomination::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

pub struct CurrencyConvert;
impl currency::CurrencyConversion<currency::Amount<Test>, CurrencyId> for CurrencyConvert {
    fn convert(
        amount: &currency::Amount<Test>,
        to: CurrencyId,
    ) -> Result<currency::Amount<Test>, sp_runtime::DispatchError> {
        let amount = convert_to(to, amount.amount())?;
        Ok(Amount::new(amount, to))
    }
}

#[cfg_attr(test, mockable)]
pub fn convert_to(to: CurrencyId, amount: Balance) -> Result<Balance, sp_runtime::DispatchError> {
    Ok(amount) // default conversion 1:1 - overwritable with mocktopus
}

impl currency::Config for Test {
    type SignedInner = SignedInner;
    type SignedFixedPoint = SignedFixedPoint;
    type UnsignedFixedPoint = UnsignedFixedPoint;
    type Balance = Balance;
    type GetNativeCurrencyId = GetNativeCurrencyId;
    type GetRelayChainCurrencyId = GetCollateralCurrencyId;
    type GetWrappedCurrencyId = GetWrappedCurrencyId;
    type CurrencyConversion = CurrencyConvert;
}

parameter_types! {
    pub const ParachainBlocksPerBitcoinBlock: BlockNumber = 100;
}

impl btc_relay::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ParachainBlocksPerBitcoinBlock = ParachainBlocksPerBitcoinBlock;
    type WeightInfo = ();
}

impl security::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

parameter_types! {
    pub const MinimumPeriod: Moment = 5;
}

impl pallet_timestamp::Config for Test {
    type Moment = Moment;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl oracle::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type OnExchangeRateChange = ();
    type WeightInfo = ();
    type MaxNameLength = ConstU32<255>;
}

parameter_types! {
    pub const FeePalletId: PalletId = PalletId(*b"mod/fees");
    pub const MaxExpectedValue: UnsignedFixedPoint = UnsignedFixedPoint::from_inner(<UnsignedFixedPoint as FixedPointNumber>::DIV);
}

impl fee::Config for Test {
    type FeePalletId = FeePalletId;
    type WeightInfo = ();
    type SignedFixedPoint = SignedFixedPoint;
    type SignedInner = SignedInner;
    type CapacityRewards = CapacityRewards;
    type VaultRewards = VaultRewards;
    type VaultStaking = VaultStaking;
    type OnSweep = ();
    type MaxExpectedValue = MaxExpectedValue;
    type NominationApi = Nomination;
}

parameter_types! {
    pub const TreasuryPalletId: PalletId = PalletId(*b"mod/trsy");
}

pub struct BlockNumberToBalance;

impl Convert<BlockNumber, Balance> for BlockNumberToBalance {
    fn convert(a: BlockNumber) -> Balance {
        a.into()
    }
}

impl Config for Test {
    type TreasuryPalletId = TreasuryPalletId;
    type RuntimeEvent = RuntimeEvent;
    type BlockNumberToBalance = BlockNumberToBalance;
    type WeightInfo = ();
}

pub type TestEvent = RuntimeEvent;
pub type TestError = Error<Test>;
pub type VaultRegistryError = vault_registry::Error<Test>;

pub const USER: AccountId = 1;
pub const VAULT: VaultId<AccountId, CurrencyId> = VaultId {
    account_id: 2,
    currencies: VaultCurrencyPair {
        collateral: DEFAULT_COLLATERAL_CURRENCY,
        wrapped: DEFAULT_WRAPPED_CURRENCY,
    },
};

pub const ALICE_BALANCE: u128 = 1_000_000;
pub const BOB_BALANCE: u128 = 1_000_000;

pub struct ExtBuilder;

impl ExtBuilder {
    pub fn build_with(balances: orml_tokens::GenesisConfig<Test>) -> sp_io::TestExternalities {
        let mut storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

        balances.assimilate_storage(&mut storage).unwrap();

        fee::GenesisConfig::<Test> {
            issue_fee: UnsignedFixedPoint::checked_from_rational(5, 1000).unwrap(), // 0.5%
            issue_griefing_collateral: UnsignedFixedPoint::checked_from_rational(5, 100000).unwrap(), // 0.005%
            redeem_fee: UnsignedFixedPoint::checked_from_rational(5, 1000).unwrap(), // 0.5%
            premium_redeem_fee: UnsignedFixedPoint::checked_from_rational(5, 100).unwrap(), // 5%
            punishment_fee: UnsignedFixedPoint::checked_from_rational(1, 10).unwrap(), // 10%
            replace_griefing_collateral: UnsignedFixedPoint::checked_from_rational(1, 10).unwrap(), // 10%
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        issue::GenesisConfig::<Test> {
            issue_period: 10,
            issue_btc_dust_value: 0,
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        const PAIR: VaultCurrencyPair<CurrencyId> = VaultCurrencyPair {
            collateral: DEFAULT_COLLATERAL_CURRENCY,
            wrapped: DEFAULT_WRAPPED_CURRENCY,
        };
        vault_registry::GenesisConfig::<Test> {
            minimum_collateral_vault: vec![(DEFAULT_COLLATERAL_CURRENCY, 0)],
            punishment_delay: 8,
            system_collateral_ceiling: vec![(PAIR, 1_000_000_000_000)],
            secure_collateral_threshold: vec![(PAIR, UnsignedFixedPoint::checked_from_rational(200, 100).unwrap())],
            premium_redeem_threshold: vec![(PAIR, UnsignedFixedPoint::checked_from_rational(120, 100).unwrap())],
            liquidation_collateral_threshold: vec![(
                PAIR,
                UnsignedFixedPoint::checked_from_rational(110, 100).unwrap(),
            )],
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        storage.into()
    }

    pub fn build() -> sp_io::TestExternalities {
        ExtBuilder::build_with(orml_tokens::GenesisConfig::<Test> {
            balances: vec![DEFAULT_COLLATERAL_CURRENCY, DEFAULT_NATIVE_CURRENCY]
                .into_iter()
                .flat_map(|currency_id| {
                    vec![
                        (USER, currency_id, ALICE_BALANCE),
                        (VAULT.account_id, currency_id, BOB_BALANCE),
                    ]
                })
                .collect(),
        })
    }
}

pub fn run_test<T>(test: T)
where
    T: FnOnce(),
{
    clear_mocks();
    ExtBuilder::build().execute_with(|| {
        assert_ok!(<oracle::Pallet<Test>>::_set_exchange_rate(
            DEFAULT_COLLATERAL_CURRENCY,
            UnsignedFixedPoint::one()
        ));
        Security::set_active_block_number(1);
        System::set_block_number(1);

        ext::btc_relay::is_fully_initialized::<Test>.mock_safe(|| MockResult::Return(Ok(true)));
        test();
    });
}
