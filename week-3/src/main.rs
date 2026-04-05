mod modules;

fn main() {
    println!("====== week 03 assignments ======");

    // mod 1  
    let order_result = modules::mod_01_orders::process_incoming_order("LIMIT", "BUY", 110, 10, 95, 105);
    println!("process_incoming_order: {}", order_result);
    let (spread, mid, notional, fee) = modules::mod_01_orders::order_economics(95, 105, 100, 10, 50);
    println!("order_economics: spread={}, mid={}, notional={}, fee={}", spread, mid, notional, fee);

    // mod 2
    let orders = vec![(100, 5), (101, 10), (99, 7), (100, 3)];
    let (bid_levels, best_bid) = modules::mod_02_orderbook::aggregate_and_best(&orders, true);
    let (ask_levels, best_ask) = modules::mod_02_orderbook::aggregate_and_best(&orders, false);
    println!("aggregate_and_best (bids): {:?}, best_bid: {}", bid_levels, best_bid);
    println!("aggregate_and_best (asks): {:?}, best_ask: {}", ask_levels, best_ask);
    let (top_bids, top_asks, bid_qty) = modules::mod_02_orderbook::book_snapshot(&bid_levels, &ask_levels, 2);
    println!("book_snapshot: top_bids={:?}, top_asks={:?}, bid_qty={}", top_bids, top_asks, bid_qty);

    // mod 3
    let match_result = modules::mod_03_buy_sell::try_match(105, 10, 100, 8);
    println!("try_match: {:?}", match_result);
    let asks = vec![(100, 5), (101, 10)];
    let fill_result = modules::mod_03_buy_sell::fill_buy_order(101, 12, &asks);
    println!("fill_buy_order: {:?}", fill_result);

    // mod 4
    let ops = vec![(true, 100), (false, 50), (false, 60)];
    let final_balance = modules::mod_04_balances::process_balance(200, &ops);
    println!("process_balance: {}", final_balance);
    let margin_ok = modules::mod_04_balances::validate_margin(true, 0, 1000, 100, 5);
    println!("validate_margin: {}", margin_ok);

    // mod 5
    let trades = vec![(100, 10), (105, 5), (98, 20), (102, 15), (101, 8)];
    let candle = modules::mod_05_market_data::build_candle(&trades);
    println!("build_candle: {:?}", candle);
}
