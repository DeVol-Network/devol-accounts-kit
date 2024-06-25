use serde::{Deserialize, Serialize};

pub const CLIENT_POOL_MAX_BASKET_LENGTH: usize = 4;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct BasketData {
    pub strike: [u8; CLIENT_POOL_MAX_BASKET_LENGTH],
    pub pc: [u8; CLIENT_POOL_MAX_BASKET_LENGTH],
    pub amount: [u8; CLIENT_POOL_MAX_BASKET_LENGTH],
}

impl Default for BasketData {
    fn default() -> Self {
        Self {
            strike: [0; CLIENT_POOL_MAX_BASKET_LENGTH],
            pc: [0; CLIENT_POOL_MAX_BASKET_LENGTH],
            amount: [0; CLIENT_POOL_MAX_BASKET_LENGTH],
        }
    }
}

impl BasketData {
    #[inline(always)]
    pub fn get_strike(&self) -> u32 { u32::from_ne_bytes(self.strike) }
    #[inline(always)]
    pub fn set_strike(&mut self, value: u32) { self.strike = value.to_ne_bytes() }
    #[inline(always)]
    pub fn get_pc(&self) -> u32 { u32::from_ne_bytes(self.pc) }
    #[inline(always)]
    pub fn set_pc(&mut self, value: u32) { self.pc = value.to_ne_bytes() }
    #[inline(always)]
    pub fn get_amount(&self) -> i32 { i32::from_ne_bytes(self.amount) }
    #[inline(always)]
    pub fn set_amount(&mut self, value: i32) { self.amount = value.to_ne_bytes() }
}