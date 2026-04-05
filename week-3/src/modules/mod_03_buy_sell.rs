pub fn try_match(buy_price: u64, buy_qty: u64, sell_price: u64, sell_qty: u64) -> (bool, u64, u64) {
    if buy_price >= sell_price {
        let trade_qty = buy_qty.min(sell_qty);
        // TODO: return the match result tuple (true, trade_price, trade_qty)
        // trade_price is the sell_price (resting order's price)
        (true, sell_price, trade_qty)
    } else {
        (false, 0, 0)
    }
}

pub fn fill_buy_order(limit_price: u64, mut qty: u64, asks: &[(u64, u64)]) -> (u64, u64, u64) {
    let mut filled = 0u64;
    let mut cost = 0u64;

    for &(ask_price, available) in asks {
        if qty == 0 {
            break;
        }

        // TODO: If this is a limit order (limit_price > 0) and the ask
        //       is too expensive (ask_price > limit_price), break
        if limit_price > 0 && (ask_price > limit_price) {
            break;
        }

        let fill = qty.min(available);
        filled += fill;

        // TODO: Add fill * ask_price to cost
        // TODO: Subtract fill from qty
        cost += fill * ask_price;
        qty -= fill;
    }

    (filled, cost, qty)
}

pub fn run_matching_engine(orders: &[(u8, u64, u64)]) -> Vec<(u64, u64)> {
    let mut bids: Vec<(u64, u64)> = Vec::new();
    let mut asks: Vec<(u64, u64)> = Vec::new();
    let mut trades: Vec<(u64, u64)> = Vec::new();

    for &(side, price, qty) in orders {
        let mut remaining = qty;

        if side == 0 {
            // BUY: match against asks
            asks.sort_by_key(|&(p, _)| p);

            while remaining > 0 && !asks.is_empty() && asks[0].0 <= price {
                let trade_price = asks[0].0;
                let trade_qty = remaining.min(asks[0].1);
                trades.push((trade_price, trade_qty));
                remaining -= trade_qty;
                asks[0].1 -= trade_qty;
                if asks[0].1 == 0 {
                    asks.remove(0);
                }
            }
            if remaining > 0 {
                bids.push((price, remaining));
            }
        } else {
            // SELL: match against bids
            bids.sort_by(|a, b| b.0.cmp(&a.0));

            // TODO: Same matching logic as BUY above, but against bids:
            //   - Match while bids[0].0 >= price
            //   - Trade at bids[0].0
            //   - Push unfilled remainder to asks
            while remaining > 0 && !bids.is_empty() && bids[0].0 <= price {
                let trade_price = bids[0].0;
                let trade_qty = remaining.min(bids[0].1);
                trades.push((trade_price, trade_qty));
                remaining -= trade_qty;
                bids[0].1 -= trade_qty;
                if bids[0].1 == 0 {
                    bids.remove(0);
                }
            }
            if remaining > 0 {
                asks.push((price, remaining));
            }
        }
    }

    trades
}
