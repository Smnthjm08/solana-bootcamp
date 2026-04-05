// Integration tests for week-3 modules

use crate::modules::mod_01_orders::{process_incoming_order, order_economics};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_incoming_order_market_buy() {
        let result = process_incoming_order("MARKET", "BUY", 100, 10, 95, 105);
        assert_eq!(result, "IMMEDIATE");
    }

    #[test]
    fn test_process_incoming_order_limit_buy_immediate() {
        let result = process_incoming_order("LIMIT", "BUY", 110, 10, 95, 105);
        assert_eq!(result, "IMMEDIATE");
    }

    #[test]
    fn test_process_incoming_order_limit_buy_resting() {
        let result = process_incoming_order("LIMIT", "BUY", 100, 10, 95, 105);
        assert_eq!(result, "RESTING");
    }

    #[test]
    fn test_order_economics() {
        let (spread, midprice, notional, fee) = order_economics(95, 105, 100, 10, 50);
        assert_eq!(spread, 10);
        assert_eq!(midprice, 100);
        assert_eq!(notional, 1000);
        assert_eq!(fee, 5);
    }
}
