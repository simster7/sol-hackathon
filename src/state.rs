use solana_program::pubkey::Pubkey;

use borsh::{BorshDeserialize, BorshSerialize};

pub struct PerpetualSwap {
    pub is_long_initialized: bool,
    pub is_short_initialized: bool,
    pub nonce: u8,
    pub token_program_id: Pubkey,
    pub long_margin_pubkey: Pubkey,
    pub long_account_pubkey: Pubkey,
    pub short_margin_pubkey: Pubkey,
    pub short_account_pubkey: Pubkey,
    pub reference_time: u128,
    pub index_price: f64,
    pub mark_price: f64,
    pub minimum_margin: u64,
    pub liquidation_threshold: f64,
    pub funding_rate: f64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
struct AccountState {
    collateral: f64,
    position: f64,
    avgEntryPrice: f64,
    leverage: f64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
struct PoolState {
    totalCollateral: f64,
    openInterest: f64,
    indexPrice: f64,
    markPrice: f64,
    fundingRate: f64, // probably a function of indexPrice, markPrice
    lastFundingTime: Datetime
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
struct FundingState {
    start_time: Datetime,
    end_time: Datetime,
    net_rate: f64,
    prev_acct: Pubkey,
    next_acct: Pubkey,
    funding_events: [FundingEvent]
    // funding events should be ordered first to last
}

struct FundingEvent{
    timestamp: Timestamp,
    rate: f64,
    funding_size: u64,
    event_pool_pos: u64,
}

impl AccountState {
    pub const LEN: usize = 32;

    pub fn is_initialized(&self) -> bool {
        self.is_long_initialized && self.is_short_initialized
    }
}

impl PerpetualSwap {
    pub const LEN: usize = 218;

    pub fn is_initialized(&self) -> bool {
        self.is_long_initialized && self.is_short_initialized
    }
}

impl FundingState {
    pub const LEN: usize = 256; //or whatever the correct # is

    pub fn new_funding_state(start_time: Datetime, prev_acct: Pubkey) -> bool {

    }

    fn query_net_funding(&self, start_time: Datetime, side: bool) -> f64 {

    }

    fn update_funding(&self, funding_rate: f64, timestamp: Datetime) -> Result<(), ProgramError> {

    }
}

mod test {
    #[cfg(test)]
    use super::*;

    #[test]
    pub fn test_perpetual_swap_unpack() {
        let p = PerpetualSwap {
            is_long_initialized: true,
            is_short_initialized: true,
            nonce: 122,
            token_program_id: Pubkey::new_unique(),
            long_margin_pubkey: Pubkey::new_unique(),
            long_account_pubkey: Pubkey::new_unique(),
            short_margin_pubkey: Pubkey::new_unique(),
            short_account_pubkey: Pubkey::new_unique(),
            reference_time: 123456900,
            index_price: 100.0,
            mark_price: 100.2,
            minimum_margin: 10,
            liquidation_threshold: 0.2,
            funding_rate: 0.8,
        };
        let packed = p.try_to_vec().unwrap();
        let unpacked = PerpetualSwap::try_from_slice(packed.as_slice()).unwrap();

        assert_eq!(p, unpacked);
    }
}
