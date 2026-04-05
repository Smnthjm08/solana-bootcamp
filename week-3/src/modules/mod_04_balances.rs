
pub fn process_balance(initial: u64, operations: &[(bool, u64)]) -> u64 {
    let mut balance = initial;
    for &(is_deposit, amount) in operations {
        if is_deposit {
            balance += amount;
        } else {
            if balance >= amount {
                balance -= amount;
            } else {
                continue;
            }
        }
    }
    balance
}

pub fn validate_margin(is_buy: bool, base_bal: u64, quote_bal: u64, price: u64, qty: u64) -> bool {
    if is_buy {
        quote_bal >= price * qty
    } else {
        base_bal >= qty
    }
}

pub fn settle_trades(
    mut buyer_base: u64, mut buyer_quote: u64,
    mut seller_base: u64, mut seller_quote: u64,
    trades: &[(u64, u64)],
) -> (u64, u64, u64, u64) {
    for &(price, qty) in trades {
        let cost = price * qty;

        // TODO: Update all four balances
          buyer_base  += qty;
          buyer_quote -= cost;
          seller_base -= qty;
          seller_quote += cost;
    }

    (buyer_base, buyer_quote, seller_base, seller_quote)
}
