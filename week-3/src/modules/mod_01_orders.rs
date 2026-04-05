// process_incoming_order: The gateway to your exchange.
//   Every order is validated, then classified.
//
// INPUT:
//   order_type: &str → "MARKET" or "LIMIT"
//   side: &str       → "BUY" or "SELL"
//   price: u64       → limit price (0 for market orders)
//   qty: u64         → quantity to trade
//   best_bid: u64    → highest resting buy price on the book
//   best_ask: u64    → lowest resting sell price on the book
//
// OUTPUT:
//   &'static str → one of: "REJECTED", "IMMEDIATE", "RESTING"
//
// STEP 1 — Validate:
//   - order_type must be "MARKET" or "LIMIT"  → else REJECTED
//   - side must be "BUY" or "SELL"            → else REJECTED
//   - qty must be > 0                         → else REJECTED
//   - if LIMIT: price must be > 0             → else REJECTED
//
// STEP 2 — Classify:
//   - MARKET orders → always IMMEDIATE
//   - LIMIT BUY  where price >= best_ask → IMMEDIATE
//   - LIMIT SELL where price <= best_bid → IMMEDIATE
//   - otherwise → RESTING
//
// EXAMPLES:
//   ("LIMIT", "BUY", 110, 10, 100, 105) → "IMMEDIATE"  (110 >= 105)
//   ("LIMIT", "BUY", 99, 10, 100, 105)  → "RESTING"    (99 < 105)
//   ("MARKET", "SELL", 0, 5, 100, 105)  → "IMMEDIATE"   (market = always)
//   ("LIMIT", "HOLD", 100, 10, 100, 105) → "REJECTED"   (bad side)
//
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

// order_economics: Computes the four key metrics for a trade.
//
// INPUT:
//   best_bid: u64  → highest resting buy price (e.g., 100)
//   best_ask: u64  → lowest resting sell price (e.g., 105)
//   price: u64     → the order's execution price (e.g., 100)
//   qty: u64       → quantity to trade (e.g., 10)
//   fee_bps: u64   → fee in basis points (e.g., 30 means 0.30%)
//                     1 bps = 1/10,000 of the value
//
// OUTPUT:
//   (u64, u64, u64, u64) → (spread, midprice, notional, fee)
//
// FORMULAS:
//   spread   = best_ask - best_bid
//   midprice = (best_bid + best_ask) / 2      (integer division)
//   notional = price * qty
//   fee      = notional * fee_bps / 10000     (integer division)
//
// EXAMPLE:
//   order_economics(100, 105, 100, 10, 30) → (5, 102, 1000, 3)
//
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
