//! OpenGov governance config

pub mod origins;
pub use origins::{
    pallet_custom_origins, ReferendumCanceller, ReferendumKiller, Spender, Treasurer,
    WhitelistedCaller,
};
mod tracks;

use frame_support::traits::EitherOf;
use frame_system::EnsureRootWithSuccess;

use super::*;

// temporary
pub type Balance = u128;
pub const UNITS: Balance = 1_000_000_000_000;
pub const CENTS: Balance = UNITS / 100;
pub const MILLICENTS: Balance = CENTS / 1_000;
pub const GRAND: Balance = CENTS * 100_000;

parameter_types! {
    pub const VoteLockingPeriod: BlockNumber = 7 * DAYS;
}

impl pallet_conviction_voting::Config for Runtime {
    type Currency = Balances;
    type MaxTurnout =
        frame_support::traits::tokens::currency::ActiveIssuanceOf<Balances, Self::AccountId>;
    type MaxVotes = ConstU32<512>;
    type Polls = Referenda;
    type RuntimeEvent = RuntimeEvent;
    type VoteLockingPeriod = VoteLockingPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const MaxBalance: Balance = Balance::max_value();
}
pub type TreasurySpender = EitherOf<EnsureRootWithSuccess<AccountId, MaxBalance>, Spender>;

impl origins::pallet_custom_origins::Config for Runtime {}

impl pallet_whitelist::Config for Runtime {
    type DispatchWhitelistedOrigin = EitherOf<EnsureRoot<Self::AccountId>, WhitelistedCaller>;
    type Preimages = Preimage;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type WhitelistOrigin = EnsureRoot<Self::AccountId>;
}

parameter_types! {
    pub const AlarmInterval: BlockNumber = 1;
    pub const SubmissionDeposit: Balance = 1 * 3 * CENTS;
    pub const UndecidingTimeout: BlockNumber = 14 * DAYS;
}

impl pallet_referenda::Config for Runtime {
    type AlarmInterval = AlarmInterval;
    type CancelOrigin = EnsureRoot<AccountId>;
    type Currency = Balances;
    type KillOrigin = EnsureRoot<AccountId>;
    type MaxQueued = ConstU32<20>;
    type Preimages = Preimage;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type Scheduler = Scheduler;
    type Slash = Treasury;
    type SubmissionDeposit = SubmissionDeposit;
    type SubmitOrigin = EnsureSigned<AccountId>;
    type Tally = pallet_conviction_voting::TallyOf<Runtime>;
    type Tracks = tracks::TracksInfo;
    type UndecidingTimeout = UndecidingTimeout;
    type Votes = pallet_conviction_voting::VotesOf<Runtime>;
    type WeightInfo = ();
}
