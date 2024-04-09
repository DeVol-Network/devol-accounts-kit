use solana_program::pubkey::Pubkey;

pub const MINT_ADDRESS_OFFSET: usize = 0;
pub const MINT_PROGRAM_ADDRESS_OFFSET: usize = 32;
pub const MINT_LOG_ADDRESS_OFFSET: usize = 64;
pub const MINT_OPS_COUNTER_OFFSET: usize = 96;
pub const MINT_TICKER_OFFSET: usize = 104;
pub const MINT_OWN_OFFSET: usize = 112;
pub const MINT_CLIENT_OFFSET: usize = 120;
pub const MINT_SIZE: usize = 128;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Mint {
    // 32 bytes, MINT_ADDRESS_OFFSET
    pub address: Pubkey,
    // 32 bytes, MINT_PROGRAM_ADDRESS_OFFSET
    pub program_address: Pubkey,
    // 32 bytes, MINT_LOG_ADDRESS_OFFSET
    pub log_address: Pubkey,
    // 8 bytes, MINT_OPS_COUNTER_OFFSET
    pub ops_counter: [u8; 8],
    // 8 bytes, MINT_TICKER_OFFSET
    pub ticker: [u8; 8],
    // 8 bytes, MINT_OWN_OFFSET
    pub own: [u8; 8],
    // 8 bytes, MINT_CLIENT_OFFSET
    pub client: [u8; 8],
}

impl Mint {
    #[inline(always)]
    pub fn get_ops_counter(&self) -> i64 { i64::from_ne_bytes(self.ops_counter) }
    #[inline(always)]
    pub fn set_ops_counter(&mut self, value: i64) { self.ops_counter = value.to_ne_bytes() }
    #[inline(always)]
    pub fn get_ticker(&self) -> i64 { i64::from_ne_bytes(self.ticker) }
    #[inline(always)]
    pub fn set_ticker(&mut self, value: i64) { self.ticker = value.to_ne_bytes() }
    #[inline(always)]
    pub fn get_own(&self) -> i64 { i64::from_ne_bytes(self.own) }
    #[inline(always)]
    pub fn set_own(&mut self, value: i64) { self.own = value.to_ne_bytes() }
    #[inline(always)]
    pub fn get_client(&self) -> i64 { i64::from_ne_bytes(self.client) }
    #[inline(always)]
    pub fn set_client(&mut self, value: i64) { self.client = value.to_ne_bytes() }
}

impl Default for Mint {
    fn default() -> Self {
        Self {
            address: Pubkey::default(),
            program_address: Pubkey::default(),
            log_address: Pubkey::default(),
            ops_counter: [0; 8],
            ticker: [0; 8],
            own: [0; 8],
            client: [0; 8],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_root_account_offsets() {
        let account = Mint::default();

        let base_ptr = &account as *const _ as usize;

        // checking fields size and offset
        assert_eq!(unsafe { &account.address as *const _ as usize } - base_ptr, MINT_ADDRESS_OFFSET);
        assert_eq!(unsafe { &account.program_address as *const _ as usize } - base_ptr, MINT_PROGRAM_ADDRESS_OFFSET);
        assert_eq!(unsafe { &account.log_address as *const _ as usize } - base_ptr, MINT_LOG_ADDRESS_OFFSET);
        assert_eq!(unsafe { &account.ops_counter as *const _ as usize } - base_ptr, MINT_OPS_COUNTER_OFFSET);
        assert_eq!(unsafe { &account.ticker as *const _ as usize } - base_ptr, MINT_TICKER_OFFSET);
        assert_eq!(unsafe { &account.own as *const _ as usize } - base_ptr, MINT_OWN_OFFSET);
        assert_eq!(unsafe { &account.client as *const _ as usize } - base_ptr, MINT_CLIENT_OFFSET);

        // checking total size
        assert_eq!(mem::size_of::<Mint>(), MINT_SIZE);
    }
}
