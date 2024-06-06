use solana_program::pubkey::Pubkey;
use crate::accounts::worker::pools_log::pool_basket::PoolBasket;
use crate::constants::{BUCKETS_COUNT};

pub const POOLS_LOG_SIZE: usize = 1312;

#[repr(C)]
#[derive(Clone, Copy)]
// Structure to save pool data for a trade. Alignment - 64 bit. Size - 1312 bytes.
pub struct PoolsLog {
    // -- Worker params --
    // Saves count of operations with the worker (including LP and option trades, start task)
    pub worker_operations_count: u64,   // 8 bytes
    // Saves option trades count for the worker
    pub pool_trades_count: u64,         // 8 bytes
    pub trade_time: i64,                // 8 bytes
    // Pool ID - number of the pool since the task started
    pub pool_id: u32,                   // 4 bytes (1/2 align)
    pub instrument_id: u32,             // 4 bytes (2/2 align)
    pub task_id: u32,                   // 4 bytes (1/2 align)
    // Fractions allow representing non-integer quantities with precision. Fractions 100 mean 0.01 minimum quantity.
    pub fractions: u32,                 // 4 bytes (2/2 align)
    // -- Trade params --
    // Public key of the client who made the trade
    pub client_pubkey: Pubkey,          // 32 bytes
    // Total cost of the trade
    pub cost: i64,                      // 8 bytes
    // Quantity (fractions) by buckets
    pub trade_quantity: [i32; BUCKETS_COUNT],       // 380 bytes (1/2 align)
    reserved: i32,                                  // 4 bytes (2/2 align)
    // Price distribution for the trade
    pub price_distribution: [i64; BUCKETS_COUNT],   // 760 bytes
    // Prices for the basket
    pub vanilla_cost: [i64; VANILLA_COST_SIZE],     // 32 bytes
    pub traded_basket: PoolBasket,      // 56 bytes
    // pub event_type: i32, // Used nowhere?
}

impl PoolsLog {
}

impl Default for PoolsLog {
    fn default() -> Self {
        Self {
            worker_operations_count: 0,
            trade_time: 0,
            fractions: 0,
            task_id: 0,
            pool_id: 0,
            instrument_id: 0,
            pool_trades_count: 0,
            client_pubkey: Pubkey::default(),
            cost: 0,
            price_distribution: [0; BUCKETS_COUNT],
            trade_quantity: [0; BUCKETS_COUNT],
            traded_basket: PoolBasket::default(),
            reserved: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::type_size_helper::align_size;
    use super::*;

    #[test]
    fn test_pools_log_offsets() {
        let real_size = std::mem::size_of::<PoolsLog>();
        assert_eq!(real_size, POOLS_LOG_SIZE);
        assert_eq!(real_size, align_size(real_size, 8));
    }
}