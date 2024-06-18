use serde::{Deserialize, Serialize};
use crate::utils::put_or_call::PutOrCall;

pub const BASKET_DATA_SIZE: usize = 24;

// Traded basket element. 64bit alignment. Size - 24 bytes.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct PoolRecordBasketData {
    pub strike: i64,            // 8 bytes
    pub vanilla_cost: i64,      // 8 bytes
    pub amount: i32,            // 4 bytes (4/8 align)
    pub put_or_call: PutOrCall, // 2 bytes (6/8 align)
    reserved: i16,              // 2 bytes (8/8 align)
}

impl Default for PoolRecordBasketData {
    fn default() -> Self {
        Self {
            strike: 0,
            vanilla_cost: 0,
            amount: 0,
            put_or_call: PutOrCall::CALL,
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
        let real_size = std::mem::size_of::<PoolRecordBasketData>();
        assert_eq!(real_size, BASKET_DATA_SIZE);
        assert_eq!(real_size, align_size(real_size, 8));
    }
}