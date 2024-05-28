use serde::{Deserialize, Serialize};
use crate::instructions_data::option_trade::{BasketData, INSTR_OPTION_TRADE_MAX_BASKET_LENGTH};

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
#[repr(C)]
// Traded basket. 32bit fields, 64bits alignment support
pub struct Basket {
    pub length: i32,
    reserved: i32, // Reserved to fit 64bit alignment
    pub basket_elements: [BasketData; INSTR_OPTION_TRADE_MAX_BASKET_LENGTH],
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