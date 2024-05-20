
pub const CLIENT_POOL_MAX_BASKET_LENGTH: usize = 4;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
#[repr(C)]
pub struct BasketData {
    pub strike: [u8; 4],
    pub pc: [u8; 4],
    pub amount: [u8; 4],
}

impl Default for BasketData {
    fn default() -> Self {
        Self {
            strike: [0; 4],
            pc: [0; 4],
            amount: [0; 4],
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