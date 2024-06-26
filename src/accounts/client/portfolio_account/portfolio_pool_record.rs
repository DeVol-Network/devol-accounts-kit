use crate::constants::{BOUNDS_COUNT, BUCKETS_COUNT};

pub const PORTFOLIO_POOL_RECORD_SIZE: usize = 8808;

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
/// 8 bytes alignment
pub struct PortfolioPoolRecord {
    pub pool_id: u32,                               // 4 bytes (4/8 bytes align)
    pub worker_id: u32,                             // 4 bytes (8/8 bytes align)
    pub instrument_id: u32,                         // 4 bytes (4/8 bytes align)
    pub fractions: u32,                             // 4 bytes (8/8 bytes align)
    pub pool_start_time: u64,                       // 8 bytes
    pub pool_expiration_time: u64,                  // 8 bytes
    pub last_trade_time: i64,                       // 8 bytes
    pub client_operations_counter: i64,          // 8 bytes
    pub pool_operations_counter: i64,            // 8 bytes
    // worker params
    pub strikes: [i64; BUCKETS_COUNT],              // 760 bytes
    pub bounds: [i64; BOUNDS_COUNT],                // 752 bytes
    // last trade info
    pub reserved: [u32; 5],                             // 4*5=20 bytes (4/8 bytes align)
    pub last_trade_quantity: [i32; BUCKETS_COUNT],      // 380 bytes (8/8 bytes align)
    pub last_price_distribution: [i64; BUCKETS_COUNT],  // 760 bytes
    // buckets trade summary
    pub depo: [i64; BUCKETS_COUNT],                 // 760 bytes
    pub cost: [i64; BUCKETS_COUNT],                 // 760 bytes
    pub result_pnl: [i64; BUCKETS_COUNT],           // 760 bytes
    // vanilla trade summary
    pub calls_quantity: [u32; BUCKETS_COUNT],       // 380 bytes (4/8 bytes align)
    pub puts_quantity: [u32; BUCKETS_COUNT],        // 380 bytes (8/8 bytes align)
    pub calls_cost: [i64; BUCKETS_COUNT],           // 760 bytes
    pub calls_result_pnl: [i64; BUCKETS_COUNT],     // 760 bytes
    pub puts_cost: [i64; BUCKETS_COUNT],            // 760 bytes
    pub puts_result_pnl: [i64; BUCKETS_COUNT],      // 760 bytes
}

impl Default for PortfolioPoolRecord {
    fn default() -> Self {
        Self {
            pool_id: 0,
            worker_id: 0,
            instrument_id: 0,
            pool_start_time: 0,
            pool_expiration_time: 0,
            fractions: 0,
            client_operations_counter: 0,
            pool_operations_counter: 0,
            last_trade_time: 0,
            depo: [0; BUCKETS_COUNT],
            cost: [0; BUCKETS_COUNT],
            result_pnl: [0; BUCKETS_COUNT],
            calls_quantity: [0; BUCKETS_COUNT],
            calls_cost: [0; BUCKETS_COUNT],
            calls_result_pnl: [0; BUCKETS_COUNT],
            puts_quantity: [0; BUCKETS_COUNT],
            puts_cost: [0; BUCKETS_COUNT],
            puts_result_pnl: [0; BUCKETS_COUNT],
            last_trade_quantity: [0; BUCKETS_COUNT],
            last_price_distribution: [0; BUCKETS_COUNT],
            strikes: [0; BUCKETS_COUNT],
            bounds: [0; BOUNDS_COUNT],
            reserved: [0; 5],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use super::*;

    #[test]
    fn test_client_pool_offsets() {
        assert_eq!(mem::size_of::<PortfolioPoolRecord>(), PORTFOLIO_POOL_RECORD_SIZE);
    }
}
