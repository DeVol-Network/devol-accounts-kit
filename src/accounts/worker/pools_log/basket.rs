use serde::{Deserialize, Serialize};
use crate::instructions_data::option_trade::{INSTR_OPTION_TRADE_MAX_BASKET_LENGTH};
use crate::utils::basket_data::BasketData;

pub const BASKET_SIZE: usize = 56;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
#[repr(C)]
// Traded basket. 32bit fields, 64bits alignment support
pub struct Basket {
    pub length: i32,    // 4 bytes
    reserved: i32,      // 4 bytes, reserved to fit 64bit alignment
    pub basket_elements: [BasketData; INSTR_OPTION_TRADE_MAX_BASKET_LENGTH], // 12x4=48 bytes
}

#[cfg(test)]
impl Default for Basket {
    fn default() -> Self {
        Self {
            length: 0,
            reserved: 0,
            basket_elements: [BasketData::default(); INSTR_OPTION_TRADE_MAX_BASKET_LENGTH],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::type_size_helper::align_size;
    use super::*;

    #[test]
    fn test_pools_log_offsets() {
        let real_size = std::mem::size_of::<Basket>();
        assert_eq!(real_size, BASKET_SIZE);
        assert_eq!(real_size, align_size(real_size, 8));
    }
}