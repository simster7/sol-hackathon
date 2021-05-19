# Perpetual

## Background
A perpetual swap is a derivative that tracks the price of an underlying asset or index by enforcing constant settlements with interest at specific funding interval. Each swap contract has a long party and a short party. Intuitively, the perpetual swap should incentivize individuals to go long (i.e. buy) when the swap trades below the index price, and it should incentivize individuals to go short (i.e. sell) when the swap trades above the index price. This mechanism is made possible by settlements.

These settlements take in the _mark price_ (the fair price of a security at particular reference point in time) of the both the underlying index and the perpetual swap. If the index is marked at _I_, the perpetual swap is marked at _P_, the funding rate is _F_% per day, and the funding interval is _T_ days: the payout protocol is roughly as follows:

```
if P < I:
	short transfers (I - P) * F * T to long
else if P > I:
	long transfers (P - I) * F * T to short
```

(A subtle but interesting note is that the payoff structure of a perpetual swap mirrors that of a future -- the time to expiration can be adjusted by manipulating _F_ and _T_)

This allows for 2 key features:
1. **Leverage**: Participants in the swap don't have to post the full collateral in order to enter their desired positions, allowing them to lever up potential gains (and losses). One important caveat of this feature is that this forces the swap contract to have a reasonably sophisticated liquidation protocol in order to stay solvent.
2. **Shorting**: Perpetual swaps are a simple way for participants to short an index without having to borrow.

Here is how the protocol is decomposed:

## State

In terms of the necessary state needed, I settled on a simple design with 3 key Account objects:
  

### Perpetual Swap Account:

This is the primary piece of state in the program. All of the instructions will interact with this account in some way. The `data` field of this account contains the following:
```
pub is_long_initialized: bool // Does the contract have a buyer
pub is_short_initialized: bool // Does the contract have a seller
pub nonce: u8
pub token_program_id: Pubkey
pub long_margin_pubkey: Pubkey // Pubkey for the long margin account (described below)
pub long_account_pubkey: Pubkey
pub short_margin_pubkey: Pubkey // Pubkey for the short margin account (described below)
pub short_account_pubkey: Pubkey
pub reference_time: u128
pub index_price: f64
pub mark_price: f64
pub minimum_margin: u64
pub liquidation_threshold: f64
pub funding_rate: f64
```  

### Long Margin Account

This is the account containing the margin funds of the party long the contract. The key of this account corresponds to `long_margin_pubkey` in PerpetualSwap

 
### Short Margin Account

This is the account containing the margin funds of the party short the contract. The key of this account corresponds to `short_margin_pubkey` in PerpetualSwap

  
### Notes
-   All of these accounts are created in the InitializePerpetualSwap (not too sure whether this is right way to do it or if the accounts should be created at different times,  **we should discuss this**)
-   The purpose of the instructions defined below is to transfer funds between the Long Margin Account, the linked account of the long user, the Short Margin account, and the linked account of the short user.
-   When the ownership of the contract changes (buy or sell on the market), we can simply just change the pubkey of the long/short account in the Perpetual Swap Account (these keys will be compared with the input accounts for auth). This accomplished by invoking `TransferLong` or `TransferShort` **(might make sense to combine into one instruction)**

## Instructions

These are the instructions I've begun to implement in `processor.rs`

### InitializePerpetualSwap
Arguments:
```
pub nonce: u8
pub funding_rate: u64 
pub minimum_margin: u64
pub liquidation_threshold: u64
``` 
This instruction initializes the perpetual swap. I think it should be called every time a user places an order into the exchange. Under the hood, `InitializePerpetualSwap` will be invoked followed by `InitializeSide` (corresponding to long if the order is a bid and short if the order is an offer)

### InitializeSide
Arguments:
```
amount_to_deposit: u64
```
This is called whenever:
1. An order is created for the first time.
2. Someone with no existing position fills a newly created order.

**Example:**
- Person A places a bid at 100_
   - Under the hood `InitializePerpetualSwap` is called followed by `InitializeSide` where the accounts being updated are the `long_margin_account` and the `long_account`
 - Person B (without an existing long position) hits (i.e. sells) Person A's bid
   - `InitializeSide` is called again, and the `short_margin_account` and `short_account` fields are updated
 - Now the swap is fully initialized!

### DepositToMargin
Arguments:
```
amount_to_deposit: u64
```
This instruction will deposit `amount_to_deposit` into the appropriate account. This should be called to reduce the amount of leverage on a position (e.g. if there's risk of being liquidated)

### WithdrawFromMargin
Arguments:
```
amount_to_withdraw: u64  
```
This instruction will withdraw `amount_to_withdraw` from the appropriate account. This should be called to increase the amount of leverage on a position.
 
### TransferLong
Arguments
```
amount: u64
```
This is called in 2 scenarios: 
1. Someone with an long position hits (i.e. sells) the best bid
   - If the best bid has no position, their newly created swap is destroyed and `TransferLong` is called.
   - If the best bid has an existing short position, that participant's _counterparty_ is the target account of `TransferLong`.
2. Someone with an existing position posts an offer and someone lifts (i.e. buys) that offer
   - If the buyer has no position, `TransferLong` is called directly.
   - If the buyer has a short position, that participant's _counterparty_ is the target account of `TransferLong`.

### TransferShort
Arguments
```
amount: u64
```
The scenarios are the same as TransferLong so I won't list them again here.

### TryToLiquidate
This one is super complicated. First we need figure out which party is at risk of liquidation by checking `mark_price - index_price`. If the at-risk party _A_ is above margin, do nothing. Otherwise, we transfer `mark_price - index_price` from _A_'s busted margin account to the other user's (_B_'s) account. Then we transfer a fee from _A_'s liquidated margin account to the insurance fund. Afterwards, we empty _B_'s margin into _B_'s user account. Finally, we empty the _A_'s margin account into _B_'s account. In the case that there are insufficient funds, this will need covered by the insurance fund. **There is undefined behavior if the insurance fund is dry.** 

#### Notes
- I think we might have to close/delete all of the accounts after all of the transfers are completed.
- There needs to be a very well defined liquidation protocol

### TransferFunds
This is essentially the implementation described in the Background section!

First, we figure out how much is owed by looking at `mark_price - index_price` (_P - I_). Then, we find the amount of time that has elapsed since `reference_time` (_T_ days). The amount owed is _|P - I| * F * T_ where _F_ is the funding rate. We then transfer that amount between the appropriate funds. If there are insufficient funds, there might be a need to liquidate, but **I'm assuming that there are enough incentives in place to perform the liquidation before that happens.**

### UpdateIndexPrice / UpdateMarkPrice
Arguments:
```
price: u64  
```

I think UpdateIndexPrice and UpdateMarkPrice might be unnecessary, but I haven't quite figured out how to use the on-chain oracle. I figured the easiest way to implement this without an oracle (seems VERY sketchy) would be to give it a `price` parameter and just have that update the index/mark price in the PerpetualSwap account data field. This is mainly just a placeholder until I figure out how to use the oracle.

Additionally, both of these functions should be atomic (otherwise, a sneaky arbitrageur can play games to try to randomly liquidate people).

