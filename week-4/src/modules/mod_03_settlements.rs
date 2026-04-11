use crate::modules::{
    mod_01_markets::{Market, WinningOutcome},
    mod_02_token_ops::UserState,
};

// set_winning_side — Declare the winner, freeze the market.
//
// Mirrors the on-chain set_winning_side instruction.
//
// VALIDATION:
//   1. caller != market.authority → Err("NotAuthority")
//      (On-chain this is an Anchor constraint, not instruction logic)
//   2. market.is_settled → Err("MarketAlreadySettled")
//   3. current_time > market.settlement_deadline → Err("MarketExpired")
//      Note: <= deadline is OK (different from split/merge which use <)
//   4. winner is not "A" or "B" → Err("InvalidWinningOutcome")
//
// STATE UPDATES:
//   market.is_settled = true
//   market.winning_outcome = Some(parsed winner)
//
fn set_winning_side(
    market: &mut Market,
    caller: &str,
    winner: &str,
    current_time: i64,
) -> Result<(), &'static str> {
    // TODO: check caller == market.authority
    // if caller != market.authority {              //commented
    //     return Err("NotAuthority");
    // }

    // TODO: check not already settled
    if market.is_settled {
        return Err("MarketAlreadySettled");
    }

    // TODO: check current_time <= settlement_deadline
    if current_time > market.settlement_deadline {
        return Err("MarketExpired");
    }

    // TODO: parse winner string to WinningOutcome, reject if invalid
    let outcome = match winner.trim().to_uppercase().as_str() {
        "A" => WinningOutcome::OutcomeA,
        "B" => WinningOutcome::OutcomeB,
        _ => return Err("InvalidWinningOutcome"),
    };

    // TODO: set is_settled and winning_outcome
    market.is_settled = true;
    market.winning_outcome = Some(outcome);

    Ok(())
}

// claim_rewards — Winners burn tokens, receive collateral.
//
// Mirrors the on-chain claim_rewards instruction.
//
// VALIDATION:
//   1. !market.is_settled → Err("MarketNotSettled")
//   2. market.winning_outcome is None → Err("WinningOutcomeNotSet")
//
// LOGIC:
//   payout = match winning_outcome:
//     OutcomeA → user.balance_a
//     OutcomeB → user.balance_b
//
//   if payout > 0:
//     checked_sub total_collateral_locked
//     zero out the winning balance (prevents double-claim)
//
// RETURNS: Ok(payout_amount)
//
fn claim_rewards(market: &mut Market, user: &mut UserState) -> Result<u64, &'static str> {
    // TODO: check market is settled
    if !market.is_settled {
        return Err("MarketNotSettled");
    }

    // TODO: check winning_outcome is Some
    let winning_outcome = match market.winning_outcome {
        Some(WinningOutcome::OutcomeA) => WinningOutcome::OutcomeA,
        Some(WinningOutcome::OutcomeB) => WinningOutcome::OutcomeB,
        _ => return Err("WinningOutcomeNotSet"),
    };

    // TODO: determine payout based on winning side
    let payout = match winning_outcome {
        WinningOutcome::OutcomeA => {
            let amount = user.balance_a;
            if amount > 0 {
                user.balance_a = 0;
            }
            amount
        }
        WinningOutcome::OutcomeB => {
            let amount = user.balance_b;
            if amount > 0 {
                user.balance_b = 0;
            }
            amount
        }
    };

    // TODO: if payout > 0, update vault and zero out user's winning balance
    if payout > 0 {
        market.total_collateral_locked = market
            .total_collateral_locked
            .checked_sub(payout)
            .ok_or("MathOverflow")?;
    }

    // TODO: return Ok(payout)
    Ok(payout)
}
