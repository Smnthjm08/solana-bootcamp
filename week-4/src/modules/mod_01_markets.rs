// initialize_market — Create a new prediction market.
//
// This mirrors the on-chain initialize_market instruction.
// Anchor handles PDA creation and rent — we handle validation and state init.
//
// INPUT:
//   market_id: u32         — unique market identifier
//   authority: &str        — the wallet that can settle this market
//   settlement_deadline: i64 — unix timestamp when betting closes
//   current_time: i64      — current block timestamp (Clock::get())
//
// OUTPUT:
//   Ok(Market) with all fields initialized, or
//   Err("InvalidSettlementDeadline") if deadline <= current_time
//

// use solana_program::pubkey::Pubkey;

pub enum WinningOutcome {
    Yes,
    No,
}

pub struct Market {
    // pub authority: Pubkey,        // who can settle this market //commented
    pub market_id: u32,                          // unique identifier
    pub settlement_deadline: i64,                // unix timestamp — when betting stops
    pub is_settled: bool,                        // has the winner been declared?
    pub winning_outcome: Option<WinningOutcome>, //commented
    pub total_collateral_locked: u64,            // running total of collateral in vault
                                                 // ... plus token mint addresses, vault address, PDA bump
}

fn initialize_market(
    market_id: u32,
    authority: &str,
    settlement_deadline: i64,
    current_time: i64,
) -> Result<Market, &'static str> {
    // The actual contract does:
    //   require!(settlement_deadline > Clock::get()?.unix_timestamp,
    //            PredictionMarketError::InvalidSettlementDeadline);

    // TODO: validate that settlement_deadline > current_time
    if settlement_deadline <= current_time {
        return Err("InvalidSettlementDeadline");
    }

    // TODO: return an initialized Market with:
    //   - is_settled = false
    //   - winning_outcome = None
    //   - total_collateral_locked = 0

    // let authority = "11111111111111111111111111111111"; // Valid base58 Pubkey

    let market = Market {
        market_id,
        // authority: authority.to_string(), //commented
        settlement_deadline,
        is_settled: false,
        winning_outcome: None, //commented
        total_collateral_locked: 0,
    };

    Ok(market)
}
