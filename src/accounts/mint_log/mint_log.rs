use solana_program::pubkey::Pubkey;

pub const MINT_LOG_ID_OFFSET: usize = 0;
pub const MINT_LOG_TIME_OFFSET: usize = 8;
pub const MINT_LOG_EVENT_TYPE_OFFSET: usize = 16;
pub const MINT_LOG_PUBKEY_OFFSET: usize = 20;
pub const MINT_LOG_OWN_OFFSET: usize = 52;
pub const MINT_LOG_CLIENT_OFFSET: usize = 60;
pub const MINT_LOG_SUM_OFFSET: usize = 68;
pub const MINT_LOG_SIZE: usize = 76;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MintLog {
    pub id: [u8; 8],
    pub time: [u8; 8],
    pub event_type: u32, // 4 bytes
    pub pubkey: Pubkey,  // 40 bytes
    pub own: [u8; 8],
    pub client: [u8; 8],
    pub sum: [u8; 8],
}

impl MintLog {
    #[inline(always)]
    pub fn get_id(&self) -> i64 { i64::from_ne_bytes(self.id) }
    #[inline(always)]
    pub fn set_id(&mut self, value: i64) { self.id = value.to_ne_bytes() }
    #[inline(always)]
    pub fn get_time(&self) -> i64 { i64::from_ne_bytes(self.time) }
    #[inline(always)]
    pub fn set_time(&mut self, value: i64) { self.time = value.to_ne_bytes() }
    #[inline(always)]
    pub fn get_own(&self) -> i64 { i64::from_ne_bytes(self.own) }
    #[inline(always)]
    pub fn set_own(&mut self, value: i64) { self.own = value.to_ne_bytes() }
    #[inline(always)]
    pub fn get_client(&self) -> i64 { i64::from_ne_bytes(self.client) }
    #[inline(always)]
    pub fn set_client(&mut self, value: i64) { self.client = value.to_ne_bytes() }
    #[inline(always)]
    pub fn get_sum(&self) -> i64 { i64::from_ne_bytes(self.sum) }
    #[inline(always)]
    pub fn set_sum(&mut self, value: i64) { self.sum = value.to_ne_bytes() }
}
impl Default for MintLog {
    fn default() -> Self {
        Self {
            id: [0; 8],
            time: [0; 8],
            event_type: 0,
            pubkey: Pubkey::default(),
            own: [0; 8],
            client: [0; 8],
            sum: [0; 8],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_root_account_offsets() {
        let account = MintLog::default();

        let base_ptr = &account as *const _ as usize;

        // checking fields size and offset
        assert_eq!(unsafe { &account.id as *const _ as usize } - base_ptr, MINT_LOG_ID_OFFSET);
        assert_eq!(unsafe { &account.time as *const _ as usize } - base_ptr, MINT_LOG_TIME_OFFSET);
        assert_eq!(unsafe { &account.event_type as *const _ as usize } - base_ptr, MINT_LOG_EVENT_TYPE_OFFSET);
        assert_eq!(unsafe { &account.pubkey as *const _ as usize } - base_ptr, MINT_LOG_PUBKEY_OFFSET);
        assert_eq!(unsafe { &account.own as *const _ as usize } - base_ptr, MINT_LOG_OWN_OFFSET);
        assert_eq!(unsafe { &account.client as *const _ as usize } - base_ptr, MINT_LOG_CLIENT_OFFSET);
        assert_eq!(unsafe { &account.sum as *const _ as usize } - base_ptr, MINT_LOG_SUM_OFFSET);

        // checking total size
        assert_eq!(mem::size_of::<MintLog>(), MINT_LOG_SIZE);
    }
}