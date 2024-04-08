use crate::constants::{BOUNDS_COUNT, BUCKETS_COUNT, VANILLA_COST_SIZE, VANILLA_MEMO_SIZE};


pub const CLIENT_POOL_ID_OFFSET: usize = 0;
pub const CLIENT_POOL_WORKER_ID_OFFSET: usize = 4;
pub const CLIENT_POOL_INSTR_ID_OFFSET: usize = 8;
pub const CLIENT_POOL_START_OFFSET: usize = 12;
pub const CLIENT_POOL_EXPIRATION_OFFSET: usize = 20;
pub const CLIENT_POOL_FRACTIONS_OFFSET: usize = 28;
pub const CLIENT_POOL_COUNTER_OFFSET: usize = 32;
pub const CLIENT_POOL_ORIG_COUNTER_OFFSET: usize = 40;
pub const CLIENT_POOL_TIME_OFFSET: usize = 48;
pub const CLIENT_POOL_DEPO_OFFSET: usize = 56;
pub const CLIENT_POOL_COST_OFFSET: usize = 816;
pub const CLIENT_POOL_RESULT_OFFSET: usize = 1576;
pub const CLIENT_POOL_CALLS_OFFSET: usize = 2336;
pub const CLIENT_POOL_CALLS_COST_OFFSET: usize = 2716;
pub const CLIENT_POOL_CALLS_RESULT_OFFSET: usize = 3476;
pub const CLIENT_POOL_PUTS_OFFSET: usize = 4236;
pub const CLIENT_POOL_PUTS_COST_OFFSET: usize = 4616;
pub const CLIENT_POOL_PUTS_RESULT_OFFSET: usize = 5376;
pub const CLIENT_POOL_LAST_COST_OFFSET: usize = 6136;
pub const CLIENT_POOL_LAST_FEES_OFFSET: usize = 6144;
pub const CLIENT_POOL_LAST_TRADE_OFFSET: usize = 6152;
pub const CLIENT_POOL_VANILLA_MEMO_OFFSET: usize = 6532;
pub const CLIENT_POOL_VANILLA_COST_OFFSET: usize = 6581;
pub const CLIENT_POOL_LAST_PX_OFFSET: usize = 6613;
pub const CLIENT_POOL_STRIKES_OFFSET: usize = 7373;
pub const CLIENT_POOL_BOUNDS_OFFSET: usize = 8133;
pub const CLIENT_POOL_SIZE: usize = 8885;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClientPool {
    pub id: u32,                                // 4 bytes, CLIENT_POOL_ID_OFFSET
    pub worker_id: u32,                         // 4 bytes, CLIENT_POOL_WORKER_ID_OFFSET
    pub instr_id: u32,                          // 4 bytes, CLIENT_POOL_INSTR_ID_OFFSET
    start: [u8; 8],                             // 8 bytes, CLIENT_POOL_START_OFFSET
    expiration: [u8; 8],                        // 8 bytes, CLIENT_POOL_EXPIRATION_OFFSET
    pub fractions: u32,                         // 4 bytes, CLIENT_POOL_FRACTIONS_OFFSET
    pub counter: i64,                           // 8 bytes, CLIENT_POOL_COUNTER_OFFSET
    pub orig_counter: i64,                      // 8 bytes, CLIENT_POOL_ORIG_COUNTER_OFFSET
    pub time: i64,                              // 8 bytes, CLIENT_POOL_TIME_OFFSET
    pub depo: [i64; BUCKETS_COUNT],             // 760 bytes, CLIENT_POOL_DEPO_OFFSET
    pub cost: [i64; BUCKETS_COUNT],             // 760 bytes, CLIENT_POOL_COST_OFFSET
    pub result: [i64; BUCKETS_COUNT],           // 760 bytes, CLIENT_POOL_RESULT_OFFSET
    pub calls: [u32; BUCKETS_COUNT],            // 380 bytes, CLIENT_POOL_CALLS_OFFSET
    calls_cost: [u8; 8*BUCKETS_COUNT],          // 760 bytes, CLIENT_POOL_CALLS_COST_OFFSET
    calls_result: [u8; 8*BUCKETS_COUNT],        // 760 bytes, CLIENT_POOL_CALLS_RESULT_OFFSET
    pub puts: [u32; BUCKETS_COUNT],             // 380 bytes, CLIENT_POOL_PUTS_OFFSET
    pub puts_cost: [i64; BUCKETS_COUNT],        // 760 bytes, CLIENT_POOL_PUTS_COST_OFFSET
    pub puts_result: [i64; BUCKETS_COUNT],      // 760 bytes, CLIENT_POOL_PUTS_RESULT_OFFSET
    pub last_cost: i64,                         // 8 bytes, CLIENT_POOL_LAST_COST_OFFSET
    pub last_fees: i64,                         // 8 bytes, CLIENT_POOL_LAST_FEES_OFFSET
    pub last_trade: [i32; BUCKETS_COUNT],       // 380 bytes, CLIENT_POOL_LAST_TRADE_OFFSET
    vanilla_memo: [u8; VANILLA_MEMO_SIZE],      // 49 bytes, CLIENT_POOL_VANILLA_MEMO_OFFSET
    vanilla_cost: [u8; 8*VANILLA_COST_SIZE],    // 32 bytes, CLIENT_POOL_VANILLA_COST_OFFSET
    last_px: [u8; 8*BUCKETS_COUNT],             // 760 bytes, CLIENT_POOL_LAST_PX_OFFSET
    strikes: [u8; 8*BUCKETS_COUNT],             // 760 bytes, CLIENT_POOL_STRIKES_OFFSET
    bounds: [u8; 8*BOUNDS_COUNT],               // 752 bytes, CLIENT_POOL_BOUNDS_OFFSET
}

impl ClientPool {
    #[inline(always)]
    pub fn get_start(&self) -> i64 { i64::from_ne_bytes(self.start) }

    #[inline(always)]
    pub fn set_start(&mut self, value: i64) { self.start = value.to_ne_bytes() }

    #[inline(always)]
    pub fn get_expiration(&self) -> i64 { i64::from_ne_bytes(self.expiration) }

    #[inline(always)]
    pub fn set_expiration(&mut self, value: i64) { self.expiration = value.to_ne_bytes() }

    #[inline(always)]
    pub fn get_calls_cost(&self, index: usize) -> i64 {
        i64::from_ne_bytes(unsafe { *(self.calls_cost[8 * index..].as_ptr() as *const [u8; 8]) })
    }

    #[inline(always)]
    pub fn set_calls_cost(&mut self, index: usize, value: i64) {
        unsafe { *(self.calls_cost.as_mut_ptr().add(8 * index) as *mut [u8; 8]) = value.to_ne_bytes(); }
    }

    #[inline(always)]
    pub fn get_calls_result(&self, index: usize) -> i64 {
        unsafe { i64::from_ne_bytes(*(self.calls_result.as_ptr().add(8 * index) as *const [u8; 8])) }
    }

    #[inline(always)]
    pub fn set_calls_result(&mut self, index: usize, value: i64) {
        unsafe { *(self.calls_result.as_mut_ptr().add(8 * index) as *mut [u8; 8]) = value.to_ne_bytes(); }
    }

    #[inline(always)]
    pub fn get_vanilla_cost(&self, index: usize) -> i64 {
        unsafe { i64::from_ne_bytes(*(self.vanilla_cost.as_ptr().add(8 * index) as *const [u8; 8])) }
    }

    #[inline(always)]
    pub fn set_vanilla_cost(&mut self, index: usize, value: i64) {
        unsafe { *(self.vanilla_cost.as_mut_ptr().add(8 * index) as *mut [u8; 8]) = value.to_ne_bytes(); }
    }

    #[inline(always)]
    pub fn get_last_px(&self, index: usize) -> i64 {
        unsafe { i64::from_ne_bytes(*(self.last_px.as_ptr().add(8 * index) as *const [u8; 8])) }
    }

    #[inline(always)]
    pub fn set_last_px(&mut self, index: usize, value: i64) {
        unsafe { *(self.last_px.as_mut_ptr().add(8 * index) as *mut [u8; 8]) = value.to_ne_bytes(); }
    }

    #[inline(always)]
    pub fn get_strikes(&self, index: usize) -> i64 {
        unsafe { i64::from_ne_bytes(*(self.strikes.as_ptr().add(8 * index) as *const [u8; 8])) }
    }

    #[inline(always)]
    pub fn set_strikes(&mut self, index: usize, value: i64) {
        unsafe { *(self.strikes.as_mut_ptr().add(8 * index) as *mut [u8; 8]) = value.to_ne_bytes(); }
    }

    #[inline(always)]
    pub fn get_bounds(&self, index: usize) -> i64 {
        unsafe { i64::from_ne_bytes(*(self.bounds.as_ptr().add(8 * index) as *const [u8; 8])) }
    }

    #[inline(always)]
    pub fn set_bounds(&mut self, index: usize, value: i64) {
        unsafe { *(self.bounds.as_mut_ptr().add(8 * index) as *mut [u8; 8]) = value.to_ne_bytes(); }
    }

}

impl Default for ClientPool {
    fn default() -> Self {
        Self {
            id: 0,
            worker_id: 0,
            instr_id: 0,
            start: [0; 8],
            expiration: [0; 8],
            fractions: 0,
            counter: 0,
            orig_counter: 0,
            time: 0,
            depo: [0; BUCKETS_COUNT],
            cost: [0; BUCKETS_COUNT],
            result: [0; BUCKETS_COUNT],
            calls: [0; BUCKETS_COUNT],
            calls_cost: [0; 8 * BUCKETS_COUNT],
            calls_result: [0; 8 * BUCKETS_COUNT],
            puts: [0; BUCKETS_COUNT],
            puts_cost: [0; BUCKETS_COUNT],
            puts_result: [0; BUCKETS_COUNT],
            last_cost: 0,
            last_fees: 0,
            last_trade: [0; BUCKETS_COUNT],
            vanilla_memo: [0; VANILLA_MEMO_SIZE],
            vanilla_cost: [0; 8 * VANILLA_COST_SIZE],
            last_px: [0; 8 * BUCKETS_COUNT],
            strikes: [0; 8 * BUCKETS_COUNT],
            bounds: [0; 8 * BOUNDS_COUNT],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use crate::utils::type_size_helper::align_size;
    use super::*;

    #[test]
    fn test_client_pool_offsets() {
        let account = ClientPool::default();

        let base_ptr = &account as *const _ as usize;

        assert_eq!(unsafe { &account.id as *const _ as usize } - base_ptr, CLIENT_POOL_ID_OFFSET);
        assert_eq!(unsafe { &account.worker_id as *const _ as usize } - base_ptr, CLIENT_POOL_WORKER_ID_OFFSET);
        assert_eq!(unsafe { &account.instr_id as *const _ as usize } - base_ptr, CLIENT_POOL_INSTR_ID_OFFSET);
        assert_eq!(unsafe { &account.start as *const _ as usize } - base_ptr, CLIENT_POOL_START_OFFSET);
        assert_eq!(unsafe { &account.expiration as *const _ as usize } - base_ptr, CLIENT_POOL_EXPIRATION_OFFSET);
        assert_eq!(unsafe { &account.fractions as *const _ as usize } - base_ptr, CLIENT_POOL_FRACTIONS_OFFSET);
        assert_eq!(unsafe { &account.counter as *const _ as usize } - base_ptr, CLIENT_POOL_COUNTER_OFFSET);
        assert_eq!(unsafe { &account.orig_counter as *const _ as usize } - base_ptr, CLIENT_POOL_ORIG_COUNTER_OFFSET);
        assert_eq!(unsafe { &account.time as *const _ as usize } - base_ptr, CLIENT_POOL_TIME_OFFSET);
        assert_eq!(unsafe { &account.depo as *const _ as usize } - base_ptr, CLIENT_POOL_DEPO_OFFSET);
        assert_eq!(unsafe { &account.cost as *const _ as usize } - base_ptr, CLIENT_POOL_COST_OFFSET);
        assert_eq!(unsafe { &account.result as *const _ as usize } - base_ptr, CLIENT_POOL_RESULT_OFFSET);
        assert_eq!(unsafe { &account.calls as *const _ as usize } - base_ptr, CLIENT_POOL_CALLS_OFFSET);
        assert_eq!(unsafe { &account.calls_cost as *const _ as usize } - base_ptr, CLIENT_POOL_CALLS_COST_OFFSET);
        assert_eq!(unsafe { &account.calls_result as *const _ as usize } - base_ptr, CLIENT_POOL_CALLS_RESULT_OFFSET);
        assert_eq!(unsafe { &account.puts as *const _ as usize } - base_ptr, CLIENT_POOL_PUTS_OFFSET);
        assert_eq!(unsafe { &account.puts_cost as *const _ as usize } - base_ptr, CLIENT_POOL_PUTS_COST_OFFSET);
        assert_eq!(unsafe { &account.puts_result as *const _ as usize } - base_ptr, CLIENT_POOL_PUTS_RESULT_OFFSET);
        assert_eq!(unsafe { &account.last_cost as *const _ as usize } - base_ptr, CLIENT_POOL_LAST_COST_OFFSET);
        assert_eq!(unsafe { &account.last_fees as *const _ as usize } - base_ptr, CLIENT_POOL_LAST_FEES_OFFSET);
        assert_eq!(unsafe { &account.last_trade as *const _ as usize } - base_ptr, CLIENT_POOL_LAST_TRADE_OFFSET);
        assert_eq!(unsafe { &account.vanilla_memo as *const _ as usize } - base_ptr, CLIENT_POOL_VANILLA_MEMO_OFFSET);
        assert_eq!(unsafe { &account.vanilla_cost as *const _ as usize } - base_ptr, CLIENT_POOL_VANILLA_COST_OFFSET);
        assert_eq!(unsafe { &account.last_px as *const _ as usize } - base_ptr, CLIENT_POOL_LAST_PX_OFFSET);
        assert_eq!(unsafe { &account.strikes as *const _ as usize } - base_ptr, CLIENT_POOL_STRIKES_OFFSET);
        assert_eq!(unsafe { &account.bounds as *const _ as usize } - base_ptr, CLIENT_POOL_BOUNDS_OFFSET);

        assert_eq!(mem::size_of::<ClientPool>(), align_size(CLIENT_POOL_SIZE, 8));
    }
}
