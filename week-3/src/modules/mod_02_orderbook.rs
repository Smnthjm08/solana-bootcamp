use std::collections::BTreeMap;

// aggregate_and_best: Builds one side of the order book from raw orders.
//
// INPUT:
//   orders: &[(u64, u64)] → list of (price, quantity) tuples
//   is_buy: bool → true for bid side, false for ask side
//
// OUTPUT:
//   (Vec<(u64, u64)>, u64) → (sorted_levels, best_price)
//     - Levels: aggregated (price, total_qty) tuples
//       Sorted DESCENDING if is_buy (highest first)
//       Sorted ASCENDING if !is_buy (lowest first)
//     - Best price: first element's price (the best)
//
// APPROACH:
//   1. Create a BTreeMap<u64, u64> (automatically sorted ascending by key)
//   2. Loop through orders, summing qty at each price:
//      for &(price, qty) in orders { *map.entry(price).or_insert(0) += qty; }
//   3. Convert to Vec:
//      - If is_buy: map.into_iter().rev().collect()  (reverse = descending)
//      - If !is_buy: map.into_iter().collect()        (ascending)
//   4. Best price = levels[0].0
//
// EXAMPLE:
//   aggregate_and_best(&[(100,10),(100,20),(101,5)], true)
//     → (vec![(101,5), (100,30)], 101)
//
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

// book_snapshot: Produces a depth-limited view of the order book.
//
// INPUT:
//   bid_levels: &[(u64, u64)] → pre-sorted DESCENDING (highest first)
//   ask_levels: &[(u64, u64)] → pre-sorted ASCENDING (lowest first)
//   depth: usize → how many levels to include from each side
//
// OUTPUT:
//   (Vec<(u64, u64)>, Vec<(u64, u64)>, u64)
//     .0 = top 'depth' bid levels
//     .1 = top 'depth' ask levels
//     .2 = imbalance percentage (0-100)
//
// IMBALANCE FORMULA:
//   Sum the quantities in the top bid levels → bid_qty
//   Sum the quantities in the top ask levels → ask_qty
//   imbalance = bid_qty * 100 / (bid_qty + ask_qty)
//
// USEFUL RUST:
//   .iter().take(n).copied().collect()  → first n elements as a new Vec
//   .iter().map(|(_, q)| q).sum()       → sum of all quantities
//
// EXAMPLE:
//   book_snapshot(
//     &[(102,50),(101,30),(100,25)],
//     &[(103,20),(104,45),(105,10)],
//     2
//   ) → (vec![(102,50),(101,30)], vec![(103,20),(104,45)], 55)
//
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
