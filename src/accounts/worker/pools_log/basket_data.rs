use serde::{Deserialize, Serialize};

pub const POOL_BASKET_LENGTH: usize = 20;
pub const BASKET_DATA_SIZE: usize = 12;

// Traded basket element. 32bit alignment. Size - 12 bytes.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct BasketData {
    pub strike: [u8; 8],        // 8 bytes
    pub amount: i32,            // 4 bytes
}

impl Default for BasketData {
    fn default() -> Self {
        Self {
            strike: [0; 8],
            amount: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::type_size_helper::align_size;
    use super::*;

    #[test]
    fn test_pools_log_offsets() {
        let real_size = std::mem::size_of::<BasketData>();
        assert_eq!(real_size, BASKET_DATA_SIZE);
        assert_eq!(real_size, align_size(real_size, 4));
    }
}