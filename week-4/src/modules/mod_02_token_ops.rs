// split_tokens — Deposit collateral, mint outcome tokens.
//
// Mirrors the on-chain split_tokens instruction.
// CPIs (token::transfer, token::mint_to) are abstracted —
// we just update the state directly.
//
// VALIDATION (in order):
//   1. market.is_settled == true → Err("MarketAlreadySettled")
//   2. current_time >= market.settlement_deadline → Err("MarketExpired")
//   3. amount == 0 → Err("InvalidAmount")
//   4. total_collateral_locked + amount overflows → Err("MathOverflow")
//
// STATE UPDATES:
//   market.total_collateral_locked += amount
//   user.balance_a += amount
//   user.balance_b += amount
//

use std::cmp::min;

use crate::modules::mod_01_markets::WinningOutcome;

pub struct Market {
    // pub authority: Pubkey,        // who can settle this market //commented
    pub market_id: u32,                          // unique identifier
    pub settlement_deadline: i64,                // unix timestamp — when betting stops
    pub is_settled: bool,                        // has the winner been declared?
    pub winning_outcome: Option<WinningOutcome>, //commented
    pub total_collateral_locked: u64,            // running total of collateral in vault
                                                 // ... plus token mint addresses, vault address, PDA bump
}

pub struct UserState {
    balance_a: u64,
    balance_b: u64,
}

fn split_tokens(
    market: &mut Market,
    user: &mut UserState,
    amount: u64,
    current_time: i64,
) -> Result<(), &'static str> {
    // TODO: check market not settled
    if market.is_settled {
        return Err("MarketAlreadySettled");
    }

    // TODO: check current_time < deadline (strictly less than)
    if current_time >= market.settlement_deadline {
        return Err("MarketExpired");
    }

    // TODO: check amount > 0
    if amount == 0 {
        return Err("InvalidAmount");
    }
    // TODO: checked_add for total_collateral_locked
    let new_total = market
        .total_collateral_locked
        .checked_add(amount)
        .ok_or("MathOverflow")?;

    // TODO: update user balances (balance_a += amount, balance_b += amount)
    let new_balance_a = user.balance_a.checked_add(amount).ok_or("MathOverflow")?;

    let new_balance_b = user.balance_b.checked_add(amount).ok_or("MathOverflow")?;

    market.total_collateral_locked = new_total;
    user.balance_a = new_balance_a;
    user.balance_b = new_balance_b;

    Ok(())
}

// merge_tokens — Burn matching pairs, reclaim collateral.
//
// Mirrors the on-chain merge_tokens instruction.
//
// VALIDATION:
//   1. market.is_settled → Err("MarketAlreadySettled")
//   2. current_time >= deadline → Err("MarketExpired")
//   3. pairs = min(balance_a, balance_b); pairs == 0 → Err("InvalidAmount")
//   4. total_collateral_locked - pairs underflows → Err("MathOverflow")
//
// STATE UPDATES:
//   market.total_collateral_locked -= pairs
//   user.balance_a -= pairs
//   user.balance_b -= pairs
//
// RETURNS: Ok(pairs) — the number of pairs burned
//
fn merge_tokens(
    market: &mut Market,
    user: &mut UserState,
    current_time: i64,
) -> Result<u64, &'static str> {
    // TODO: check market not settled
    if market.is_settled {
        return Err("MarketAlreadySettled");
    }

    // TODO: check before deadline
    if current_time >= market.settlement_deadline {
        return Err("MarketExpired");
    }

    // TODO: calculate pairs = min(balance_a, balance_b)
    let pairs = min(user.balance_a, user.balance_b);

    // TODO: check pairs > 0
    if pairs == 0 {
        return Err("InvalidAmount");
    }

    // TODO: checked_sub for total_collateral_locked
    let new_total = market
        .total_collateral_locked
        .checked_sub(pairs)
        .ok_or("MathOverflow")?;

    // TODO: update user balances
    market.total_collateral_locked = new_total;
    user.balance_a -= pairs;
    user.balance_b -= pairs;

    // TODO: return Ok(pairs)
    Ok(pairs)
}
