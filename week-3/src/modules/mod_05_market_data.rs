
pub fn build_candle(trades: &[(u64, u64)]) -> (u64, u64, u64, u64, u64, u64) {
    let open = trades[0].0;
    let close = trades[trades.len() - 1].0;
    let high = trades.iter().map(|(p, _)| *p).max().unwrap();

    // TODO: Calculate low — minimum price across all trades
    let low: u64 = trades.iter().map(|(p, _)| *p).min().unwrap();

    // TODO: Calculate volume — sum of all quantities
    let volume: u64 = trades.iter().map(|(_, q)| *q).sum();

    // TODO: Calculate vwap — sum(price * qty) / volume
    //   Hint: let total_value: u64 = trades.iter().map(|(p, q)| p * q).sum();
    //         then total_value / volume
    let total_value: u64 = trades.iter().map(|(p, q)| p * q).sum();
    let vwap: u64 = total_value / volume;

    (open, high, low, close, volume, vwap)
}

// analyze_volume: Splits trade volume by aggressor side.
//
// INPUT:
//   trades: &[(u64, u8)] → list of (quantity, aggressor_side)
//     side 0 = taker-buy (buyer was aggressor)
//     side 1 = taker-sell (seller was aggressor)
//
// OUTPUT:
//   (u64, u64, u64) → (taker_buy_vol, taker_sell_vol, buy_percentage)
//     buy_percentage = buy_vol * 100 / (buy_vol + sell_vol)
//
// EXAMPLE:
//   analyze_volume(&[(100, 0), (50, 1), (75, 0), (25, 1)])
//     buy_vol = 175, sell_vol = 75
//     buy_pct = 175 * 100 / 250 = 70
//     → (175, 75, 70)
//
pub fn analyze_volume(trades: &[(u64, u8)]) -> (u64, u64, u64) {
    let mut buy_vol = 0u64;
    let mut sell_vol = 0u64;

    for &(qty, side) in trades {
        // TODO: if side == 0, add to buy_vol; else add to sell_vol
        if side == 0 {
            buy_vol += qty;
        } else {
            sell_vol += qty;
        }
    }

    // TODO: calculate buy_pct = buy_vol * 100 / (buy_vol + sell_vol)
    let buy_pct: u64 = buy_vol * 100 / (buy_vol + sell_vol);

    (buy_vol, sell_vol, buy_pct)
}
