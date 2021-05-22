use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::PerpetualSwapError;
pub enum PerpetualSwapInstruction {

    /*
        Should called only one time. Creates the global Pool State for the perpetual swap
        Might interact with Serum
    */
    InitializePerpetualSwap {
        nonce: u8,
        funding_rate: f64,
        minimum_margin: u64,
        liquidation_threshold: f64,
    },

    /*
        All Position updating logic will be handled in this instruction
        Might interact with Serum
    */
    UpdatePosition {
        position: f64,
    },

    /*
        Increases the `collateral` in an AccountState. The effect of this is decreasing leverage
    */
    DepositToMargin { amount_to_deposit: u64 },

    /*
        Decreases the `collateral` in an AccountState. The effect of this is increasing leverage
    */
    WithdrawFromMargin { amount_to_withdraw: u64 },

    /*
        External actors should incentivized to invoke this instruction in order to take on the position
        of an account that exceeded its margin requirements. This external party will receive a "bounty"
        as compensation for the risk, and the liquidated account position will be tranferred to said party
    */
    TryToLiquidate {},

    /*
        The global PoolState will be updated to reflect the new difference between `indexPrice` and 
        `markPrice`. As a result of this, everyone with an active position will be entitled to a updated
        balance.
    */
    TransferFunds {},

    /*
        A external application will invoke this instruction to update the index and mark prices
        TODO: fetch this prices from an on-chain oracle to remove existing parameters
    */
    UpdatePrices {
        index_price: f64,
        mark_price: f64,
    },

}

impl PerpetualSwapInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(PerpetualSwapError::InvalidInstruction)?;

        Ok(match tag {
            0 => {
                let (&nonce, rest) = rest
                    .split_first()
                    .ok_or(PerpetualSwapError::InvalidInstruction)?;
                let (funding_rate, rest) = Self::unpack_f64(rest)?;
                let (minimum_margin, rest) = Self::unpack_u64(rest)?;
                let (liquidation_threshold, _rest) = Self::unpack_f64(rest)?;
                Self::InitializePerpetualSwap {
                    nonce,
                    funding_rate,
                    minimum_margin,
                    liquidation_threshold,
                }
            }
            1 => {
                let (amount_to_deposit, _rest) = Self::unpack_u64(rest)?;
                Self::InitializeSide { amount_to_deposit }
            }
            2 => {
                let (amount_to_deposit, _rest) = Self::unpack_u64(rest)?;
                Self::DepositToMargin { amount_to_deposit }
            }
            3 => {
                let (amount_to_withdraw, _rest) = Self::unpack_u64(rest)?;
                Self::WithdrawFromMargin { amount_to_withdraw }
            }
            4 => Self::TryToLiquidate {},
            5 => Self::TransferFunds {},
            6 => {
                let (price, _rest) = Self::unpack_f64(rest)?;
                Self::UpdateIndexPrice { price }
            }
            7 => {
                let (price, _rest) = Self::unpack_f64(rest)?;
                Self::UpdateMarkPrice { price }
            }
            _ => return Err(PerpetualSwapError::InvalidInstruction.into()),
        })
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(PerpetualSwapError::InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(PerpetualSwapError::InvalidInstruction.into())
        }
    }

    fn unpack_f64(input: &[u8]) -> Result<(f64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(f64::from_le_bytes)
                .ok_or(PerpetualSwapError::InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(PerpetualSwapError::InvalidInstruction.into())
        }
    }
}
