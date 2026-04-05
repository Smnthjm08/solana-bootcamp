
pub fn process_incoming_order(
    order_type: &str,
    side: &str,
    price: u64,
    qty: u64,
    best_bid: u64,
    best_ask: u64,
) -> &'static str {
    // --- STEP 1: Validation ---
    // Check order_type is "MARKET" or "LIMIT"
    if order_type != "MARKET" && order_type != "LIMIT" {
        return "REJECTED";
    }

    // TODO: Check side is "BUY" or "SELL" — reject otherwise
    if side != "BUY" && side != "SELL" {
        return "REJECTED";
    }

    // TODO: Check qty > 0 — reject otherwise
    if qty == 0 {
        return "REJECTED";
    }

    // TODO: If LIMIT order, check price > 0 — reject otherwise
    if order_type == "LIMIT" && price <= 0 {
        return "REJECTED";
    }

    // --- STEP 2: Classification ---
    // TODO: Market orders are always IMMEDIATE
    if order_type == "MARKET" {
        return "IMMEDIATE";
    }

    // TODO: Limit buy — IMMEDIATE if price >= best_ask
    if order_type == "LIMIT" && side == "BUY" && price >= best_ask {
        return "IMMEDIATE";
    }

    // TODO: Limit sell — IMMEDIATE if price <= best_bid
    if order_type == "LIMIT" && side == "SELL" && price <= best_bid {
        return "IMMEDIATE";
    }

    // If nothing matched above, the order rests on the book
    "RESTING"
}

pub fn order_economics(
    best_bid: u64,
    best_ask: u64,
    price: u64,
    qty: u64,
    fee_bps: u64,
) -> (u64, u64, u64, u64) {
    let spread = best_ask - best_bid;

    // TODO: calculate midprice — average of best_bid and best_ask (integer division)
    let midprice = (best_bid + best_ask) / 2;

    // TODO: calculate notional — total value of the trade
    let notional = price * qty;

    // TODO: calculate fee — exchange's cut in basis points
    let fee = notional * fee_bps / 10000;

    (spread, midprice, notional, fee)
}

// sort_buy_orders: Sort buy-side orders by priority.
//   Rule: HIGHEST price first. Ties broken by EARLIEST timestamp.
//
// sort_sell_orders: Sort sell-side orders by priority.
//   Rule: LOWEST price first. Ties broken by EARLIEST timestamp.
//
// INPUT:
//   orders: &[(u64, u64)] → list of (price, timestamp) tuples
//
// OUTPUT:
//   Vec<usize> → indices into the original list, in priority order
//
// APPROACH:
//   1. Create a Vec of indices: (0..orders.len()).collect()
//   2. Sort those indices with sort_by() using a custom comparator
//   3. For buys: compare b's price vs a's price (descending)
//      then a's timestamp vs b's timestamp (ascending)
//   4. Use .cmp() to compare and .then() to chain
//
// EXAMPLE:
//   sort_buy_orders(&[(100, 1), (101, 2), (100, 3)]) → vec![1, 0, 2]
//
pub fn sort_buy_orders(orders: &[(u64, u64)]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..orders.len()).collect();

    indices.sort_by(|&a, &b| {
        // Primary: highest price first → compare b's price vs a's price
        // Secondary: earliest time first → compare a's time vs b's time
        orders[b]
            .0
            .cmp(&orders[a].0)
            .then(orders[a].1.cmp(&orders[b].1))
    });

    indices
}

pub fn sort_sell_orders(orders: &[(u64, u64)]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..orders.len()).collect();

    indices.sort_by(|&a, &b| {
        // TODO: LOWEST price first (ascending), then earliest timestamp
        // Hint: for ascending price, compare a's price vs b's price
        //       orders[a].0.cmp(&orders[b].0)
        //       then chain with .then(orders[a].1.cmp(&orders[b].1))
        orders[a]
            .0
            .cmp(&orders[b].0)
            .then(orders[a].1.cmp(&orders[b].1))
    });

    indices
}
