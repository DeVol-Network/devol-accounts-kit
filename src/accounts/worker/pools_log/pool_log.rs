use solana_program::pubkey::Pubkey;
use crate::accounts::worker::pools_log::basket::Basket;
use crate::constants::{BUCKETS_COUNT, VANILLA_COST_SIZE};

#[repr(C)]
#[derive(Clone, Copy)]
// Structure to save pool data for a trade. Alignment - 64 bit.
pub struct PoolsLog {
    // -- Worker params --
    // Log index in the pool logs array
    pub log_id: i64,
    pub trade_time: i64,
    pub counter: u64,
    // Pool ID - number of the pool since the task started
    pub pool_id: i32,
    pub instrument_id: i32,
    pub task_id: i32,
    // Fractions allow representing non-integer quantities with precision. Fractions 100 mean 0.01 minimum quantity.
    pub fractions: i32,
    // -- Trade params --
    // Public key of the client who made the trade
    pub pubkey: Pubkey,
    // Total cost of the trade
    pub cost: i64,
    // Price distribution for the trade
    pub price_distribution: [i64; BUCKETS_COUNT],
    // Prices for the basket
    pub vanilla_cost: [i64; VANILLA_COST_SIZE],
    // pub event_type: i32, // Used nowhere?
    // Quantity (fractions) by buckets
    pub trade_quantity: [i32; BUCKETS_COUNT], // Unaligned - need a 32bit variable near
    pub traded_basket: Basket,
}

impl PoolsLog {
}

#[cfg(test)]
impl Default for PoolsLog {
    fn default() -> Self {
        Self {
            log_id: 0,
            trade_time: 0,
            fractions: 0,
            task_id: 0,
            pool_id: 0,
            instrument_id: 0,
            counter: 0,
            pubkey: Pubkey::default(),
            cost: 0,
            price_distribution: [0; BUCKETS_COUNT],
            trade_quantity: [0; BUCKETS_COUNT],
            vanilla_cost: [0; VANILLA_COST_SIZE],
            traded_basket: Basket::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pools_log_offsets() {
        let log = PoolsLog::default();
        let base_ptr = &log as *const _ as usize;
    }
}