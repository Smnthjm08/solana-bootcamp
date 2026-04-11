use std::cmp::min;

use crate::modules::{
    mod_01_markets::{Market, WinningOutcome},
    mod_02_token_ops::UserState,
};

fn split_tokens(user: &mut UserState, amount: u64) -> Result<(), &'static str> {
    let new_balance_a = user.balance_a.checked_add(amount).ok_or("MathOverflow")?;
    let new_balance_b = user.balance_b.checked_add(amount).ok_or("MathOverflow")?;

    user.balance_a = new_balance_a;
    user.balance_b = new_balance_b;

    Ok(())
}

fn merge_tokens(user: &mut UserState) -> Result<u64, &'static str> {
    let pairs = min(user.balance_a, user.balance_b);

    user.balance_a = user.balance_a.checked_sub(pairs).ok_or("MathOverflow")?;
    user.balance_b = user.balance_b.checked_sub(pairs).ok_or("MathOverflow")?;

    Ok(pairs)
}

fn set_winning_side(caller: &str, winner: &str, market: &mut Market) -> Result<(), &'static str> {
    if caller != market.authority {
        return Err("NotAuthority");
    }
    if market.is_settled {
        return Err("MarketAlreadySettled");
    }

    market.winning_outcome = Some(match winner {
        "A" => WinningOutcome::OutcomeA,
        "B" => WinningOutcome::OutcomeB,
        _ => return Err("InvalidWinner"),
    });

    market.is_settled = true;
    Ok(())
}

fn claim_rewards(user: &mut UserState, market: &mut Market) -> Result<u64, &'static str> {
    let outcome = match &market.winning_outcome {
        Some(o) => o,
        None => return Err("WinningOutcomeNotSet"),
    };

    let payout = match outcome {
        WinningOutcome::OutcomeA => user.balance_a,
        WinningOutcome::OutcomeB => user.balance_b,
    };

    market.total_collateral_locked = market
        .total_collateral_locked
        .checked_sub(payout)
        .ok_or("MathOverflow")?;

    user.balance_a = 0;
    user.balance_b = 0;

    Ok(payout)
}

fn simulate_market(n_users: usize, authority: &str, deadline: i64, ops: &[&str]) -> String {
    let mut market = Market {
        market_id: 1,
        authority: authority.to_string(),
        settlement_deadline: deadline,
        is_settled: false,
        winning_outcome: None,
        total_collateral_locked: 0,
    };
    let mut users: Vec<UserState> = (0..n_users)
        .map(|_| UserState {
            balance_a: 0,
            balance_b: 0,
        })
        .collect();
    let mut claimed = vec![0u64; n_users];

    for op in ops {
        let parts: Vec<&str> = op.split(':').collect();

        if parts[0] == "S" {
            let user: usize = parts[1].parse().unwrap();
            let amount: u64 = parts[2].parse().unwrap();
            let time: i64 = parts[3].parse().unwrap();

            if !market.is_settled && time < market.settlement_deadline && amount > 0 {
                let user_state = &mut users[user];

                market.total_collateral_locked = market
                    .total_collateral_locked
                    .checked_add(amount)
                    .ok_or("MathOverflow")
                    .unwrap();

                split_tokens(user_state, amount);
            }
        } else if parts[0] == "M" {
            let user: usize = parts[1].parse().unwrap();
            let time: i64 = parts[2].parse().unwrap();

            if !market.is_settled && time < market.settlement_deadline {
                let user_state = &mut users[user];

                let pairs = merge_tokens(user_state).unwrap();

                market.total_collateral_locked = market
                    .total_collateral_locked
                    .checked_sub(pairs)
                    .ok_or("MathOverflow")
                    .unwrap();
            }
        } else if parts[0] == "SETTLE" {
            let caller = parts[1];
            let winner = parts[2];
            let time: i64 = parts[3].parse().unwrap();
            // TODO: implement settle logic
            //   - check authority, not settled, time <= deadline, valid winner
            //   - set is_settled and winning_outcome
            if market.authority == caller && !market.is_settled && time <= deadline {
                let _ = set_winning_side(caller, winner, &mut market).unwrap();
            }
        } else if parts[0] == "CLAIM" {
            let user: usize = parts[1].parse().unwrap();
            // TODO: implement claim logic
            //   - check settled, winning_outcome set
            //   - determine payout, checked_sub, zero balance, track claimed
            if market.is_settled && market.winning_outcome.is_some() {
                let user_state = &mut users[user];
                let payout = claim_rewards(user_state, &mut market).unwrap();
                claimed[user] = payout;
            }
        }
    }

    let mut result = market.total_collateral_locked.to_string();
    for c in &claimed {
        result.push(' ');
        result.push_str(&c.to_string());
    }
    result
}
