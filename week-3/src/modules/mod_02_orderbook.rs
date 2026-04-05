use std::collections::BTreeMap;

pub fn aggregate_and_best(orders: &[(u64, u64)], is_buy: bool) -> (Vec<(u64, u64)>, u64) {
    let mut map = BTreeMap::new();

    for &(price, qty) in orders {
        *map.entry(price).or_insert(0u64) += qty;
    }

    // TODO: Convert the map to a Vec of (price, qty) tuples
    //   If is_buy: reverse the iterator for descending order
    //   If !is_buy: keep ascending order
    // Then extract the best price from levels[0].0
    let levels: Vec<(u64, u64)> = if is_buy {
        map.into_iter().rev().collect()
    } else {
        map.into_iter().collect()
    };

    let best_price = levels.first().map(|(p, _)| *p).unwrap_or(0);
    (levels, best_price)
}

pub fn book_snapshot(
    bid_levels: &[(u64, u64)],
    ask_levels: &[(u64, u64)],
    depth: usize,
) -> (Vec<(u64, u64)>, Vec<(u64, u64)>, u64) {
    // Step 1: Take the top 'depth' levels from each side
    let top_bids: Vec<(u64, u64)> = bid_levels.iter().take(depth).copied().collect();
    let top_asks: Vec<(u64, u64)> = ask_levels.iter().take(depth).copied().collect();

    // Step 2: Sum quantities on each side
    // TODO: Calculate bid_qty (sum of quantities in top_bids)
    // Hint: top_bids.iter().map(|(_, q)| q).sum()
    let bid_qty: u64 = top_bids.iter().map(|(_, q)| q).sum();

    // TODO: Calculate ask_qty (sum of quantities in top_asks)
    let ask_qty: u64 = top_asks.iter().map(|(_, q)| q).sum();

    // Step 3: Calculate imbalance
    // TODO: imbalance = bid_qty * 100 / (bid_qty + ask_qty)
    let imbalance: u64 = bid_qty * 100 / (bid_qty + ask_qty);

    (top_bids, top_asks, imbalance)
}
