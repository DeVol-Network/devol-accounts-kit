use serde::{Deserialize, Serialize};
use crate::accounts::worker::pools_log::basket_data::BasketData;
use crate::instructions_data::option_trade::{INSTR_OPTION_TRADE_MAX_BASKET_LENGTH};
use crate::utils::bitmask::BitMask;
use crate::utils::put_or_call::PutOrCall;

pub const BASKET_SIZE: usize = 56;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
#[repr(C)]
// Traded basket. 32bit fields, 64bits alignment support. Size - 56 bytes.
pub struct PoolBasket {
    pub length: i32,                    // 4 bytes (1/2)
    put_or_call_mask: BitMask<u32>,     // 4 bytes (2/2)
    pub basket_elements: [BasketData; INSTR_OPTION_TRADE_MAX_BASKET_LENGTH], // 12x4=48 bytes
}

impl PoolBasket {
    #[inline(always)]
    pub fn get_basket_put_or_call(&self, basket_id: usize) -> PutOrCall {
        if self.put_or_call_mask.get(basket_id as u8) {
            PutOrCall::PUT
        } else {
            PutOrCall::CALL
        }
    }

    #[inline(always)]
    pub fn set_basket_put_or_call(&mut self, basket_id: usize, put_or_call: PutOrCall) {
        if put_or_call == PutOrCall::PUT {
            self.put_or_call_mask.set(basket_id as u8, true);
        } else {
            self.put_or_call_mask.set(basket_id as u8, false);
        }
    }
}

impl Default for PoolBasket {
    fn default() -> Self {
        Self {
            length: 0,
            put_or_call_mask: BitMask(0u32),
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
        let real_size = std::mem::size_of::<PoolBasket>();
        assert_eq!(real_size, BASKET_SIZE);
        assert_eq!(real_size, align_size(real_size, 8));
    }
}