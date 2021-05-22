use solana_program::pubkey::Pubkey;

use borsh::{BorshDeserialize, BorshSerialize};

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

impl AccounState {
    pub const LEN: usize = 32;

    pub fn is_initialized(&self) -> bool {
        self.is_long_initialized && self.is_short_initialized
    }
}

impl PoolState {
    pub const LEN: usize = 48;

    pub fn is_initialized(&self) -> bool {
        self.is_long_initialized && self.is_short_initialized
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
