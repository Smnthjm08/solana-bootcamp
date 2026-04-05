// try_match: Checks if a buy and sell order can trade.
//
// INPUT:
//   buy_price: u64   → max price the buyer will pay
//   buy_qty: u64     → how many units the buyer wants
//   sell_price: u64  → min price the seller will accept
//   sell_qty: u64    → how many units the seller offers
//
// OUTPUT:
//   (bool, u64, u64) → (matched, trade_price, trade_qty)
//   If no match: (false, 0, 0)
//
// RULES:
//   - Match if buy_price >= sell_price
//   - Trade price = sell_price (resting order's price)
//   - Trade qty = min(buy_qty, sell_qty)
//
// EXAMPLE:
//   try_match(105, 10, 100, 5) → (true, 100, 5)
//   try_match(99, 10, 100, 5)  → (false, 0, 0)
//
pub fn try_match(buy_price: u64, buy_qty: u64, sell_price: u64, sell_qty: u64) -> (bool, u64, u64) {
    if buy_price >= sell_price {
        let trade_qty = buy_qty.min(sell_qty);
        // TODO: return the match result tuple (true, trade_price, trade_qty)
        // trade_price is the sell_price (resting order's price)
        todo!()
    } else {
        (false, 0, 0)
    }
}
