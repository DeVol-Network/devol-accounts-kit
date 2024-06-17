use serde::{Deserialize, Serialize};
use crate::accounts::worker::pool_logs::basket_data::{PoolRecordBasketData};

pub const POOL_RECORD_BASKET_SIZE: usize = 104;
pub const POOL_RECORD_BASKET_LENGTH: usize = 4;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
#[repr(C)]
// Traded basket. 32bit fields, 64bits alignment support. Size - 104 bytes.
pub struct PoolRecordBasket {
    pub length: i64,                    // 8 bytes
    pub basket_elements: [PoolRecordBasketData; POOL_RECORD_BASKET_LENGTH], // 24x4=96 bytes
}

impl Default for PoolRecordBasket {
    fn default() -> Self {
        Self {
            length: 0,
            basket_elements: [PoolRecordBasketData::default(); POOL_RECORD_BASKET_LENGTH],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::type_size_helper::align_size;
    use super::*;

    #[test]
    fn test_pools_log_offsets() {
        let real_size = std::mem::size_of::<PoolRecordBasket>();
        assert_eq!(real_size, POOL_RECORD_BASKET_SIZE);
        assert_eq!(real_size, align_size(real_size, 8));
    }
}