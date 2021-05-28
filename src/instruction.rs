use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::PerpetualSwapError;
use crate::traits::Unpackable;

pub enum PerpetualSwapInstruction {
    /// Accounts expected:
    /// 0. `[w, signer]` New PerpetualSwap to create.
    /// 1. `[]` swap authority derived from `create_program_address(&[Token-swap account])`
    /// 2. `[]` long margin account
    /// 3. `[]` long user account
    /// 4. `[]` short margin account
    /// 5. `[]` short user account
    /// 6. `[w]` Pool Token Mint. Must be empty, owned by swap authority.
    /// 7. `[w]` Pool Token Account to deposit trading and withdraw fees.
    /// Must be empty, not owned by swap authority
    /// 8. `[w]` Pool Token Account to deposit the initial pool token
    /// supply.  Must be empty, not owned by swap authority.
    /// 9. `[]` Token program id
    InitializePerpetualSwap {
        nonce: u8,
        funding_rate: f64,
        minimum_margin: f64,
        liquidation_bounty: f64,
        minimum_funding_period: u128,
    },

    /// Accounts expected:
    /// 0. `[w]` PerpetualSwap
    /// 1. `[]` swap authority
    /// 2. `[]` user transfer authority
    /// 3. `[w, s]` The account of the person depositing to the margin account
    /// 4. `[w]` The margin account
    /// 5. `[]` The token program
    InitializeSide { amount_to_deposit: u64 },

    /// Accounts expected:
    /// 0. `[]` PerpetualSwap
    /// 1. `[]` swap authority
    /// 2. `[]` user transfer authority
    /// 3. `[w, s]` The account of the person depositing to the margin account
    /// 4. `[w]` The margin account
    /// 5. `[]` The token program
    DepositToMargin { amount_to_deposit: u64 },

    /// Accounts expected:
    /// 0. `[]` PerpetualSwap
    /// 1. `[]` swap authority
    /// 2. `[]` user transfer authority
    /// 3. `[w]` The account of the person withdrawing from the margin account
    /// 4. `[w, s]` The margin account
    /// 5. `[]` The token program
    WithdrawFromMargin { amount_to_withdraw: u64 },

    /// Accounts expected:
    /// 0. `[]` PerpetualSwap
    /// 1. `[]` swap authority
    /// 2. `[]` user transfer authority
    /// 3. `[w]` The margin account of the long party who is selling
    /// 4. `[w]` The user account of the long party who is selling
    /// 5. `[w]` The account of the party who is buying
    /// 6. `[]` The token program
    TransferLong { amount: u64 },

    /// Accounts expected:
    /// 0. `[w]` PerpetualSwap
    /// 1. `[]` swap authority
    /// 2. `[]` user transfer authority
    /// 3. `[w]` The account of the short party who is buying
    /// 4. `[w]` The account of the party who is selling
    /// 5. `[w]` The new margin account of the party who is selling
    /// 7. `[]` The token program
    TransferShort { amount: u64 },

    /// Accounts expected:
    /// 0. `[]` PerpetualSwap
    /// 1. `[]` swap authority
    /// 2. `[]` user transfer authority
    /// 3. `[w]` The account of the party to be liquidated
    /// 4. `[w]` The account of the counterparty
    /// 5. `[w]` The margin account of the party to be liquidated
    /// 6. `[w]` The insurance fund
    /// 8. `[]` The token program
    TryToLiquidate { collateral: u64 },

    /// Accounts expected:
    /// 0. `[w]` PerpetualSwap (w because reference time needs to be updated)
    /// 1. `[]` swap authority
    /// 2. `[]` user transfer authority
    /// 3. `[w]` The account of the party who is long
    /// 4. `[w]` The account of the party who is short
    /// 3. `[]` The token program
    TransferFunds {},

    /// Accounts expected:
    /// 0. `[]` PerpetualSwap
    /// 1. `[]` swap authority
    /// 2. `[]` user transfer authority
    /// 3. `[]` The token program
    UpdatePrices { index_price: f64, mark_price: f64 },
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
                let (funding_rate, rest) = Self::unpack_fn::<f64>(rest)?;
                let (minimum_margin, rest) = Self::unpack_fn::<f64>(rest)?;
                let (liquidation_bounty, rest) = Self::unpack_fn::<f64>(rest)?;
                let (minimum_funding_period, _rest) = Self::unpack_fn::<u128>(rest)?;
                Self::InitializePerpetualSwap {
                    nonce,
                    funding_rate,
                    minimum_margin,
                    liquidation_bounty,
                    minimum_funding_period,
                }
            }
            1 => {
                let (amount_to_deposit, _rest) = Self::unpack_fn::<u64>(rest)?;
                Self::InitializeSide { amount_to_deposit }
            }
            2 => {
                let (amount_to_deposit, _rest) = Self::unpack_fn::<u64>(rest)?;
                Self::DepositToMargin { amount_to_deposit }
            }
            3 => {
                let (amount_to_withdraw, _rest) = Self::unpack_fn::<u64>(rest)?;
                Self::WithdrawFromMargin { amount_to_withdraw }
            }
            4 => {
                let (collateral, _rest) = Self::unpack_fn::<u64>(rest)?;
                Self::TryToLiquidate { collateral }
            }
            5 => Self::TransferFunds {},
            6 => {
                let (index_price, _rest) = Self::unpack_fn::<f64>(rest)?;
                let (mark_price, _rest) = Self::unpack_fn::<f64>(rest)?;

                Self::UpdatePrices {
                    index_price,
                    mark_price,
                }
            }
            _ => return Err(PerpetualSwapError::InvalidInstruction.into()),
        })
    }

    fn unpack_fn<T: Unpackable>(input: &[u8]) -> Result<(T, &[u8]), ProgramError> {
        if input.len() >= T::get_bytes() {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..T::get_bytes())
                .and_then(|slice| slice.try_into().ok())
                .map(T::from_le_bytes)
                .ok_or(PerpetualSwapError::InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(PerpetualSwapError::InvalidInstruction.into())
        }
    }
}
