use serde::{Deserialize, Serialize};
use crate::utils::put_or_call::PutOrCall;

pub const OPTION_TRADE_BASKET_DATA_SIZE: usize = 8;

// Traded basket element. 32bit alignment. Size - 12 bytes.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct OptionTradeBasketData {
    pub strike_id: u16,         // 2 bytes (2/4 bytes align)
    pub put_or_call: PutOrCall, // 2 bytes (4/4 bytes align)
    pub amount: i32,            // 4 bytes
}

impl Default for OptionTradeBasketData {
    fn default() -> Self {
        Self {
            strike_id: 0,
            put_or_call: PutOrCall::PUT,
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
        let real_size = std::mem::size_of::<OptionTradeBasketData>();
        assert_eq!(real_size, OPTION_TRADE_BASKET_DATA_SIZE);
        assert_eq!(real_size, align_size(real_size, 4));
    }
}