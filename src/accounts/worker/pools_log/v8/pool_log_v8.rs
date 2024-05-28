use solana_program::pubkey::Pubkey;
use crate::constants::{BUCKETS_COUNT, VANILLA_COST_SIZE, VANILLA_MEMO_SIZE};
pub const POOLS_LOG_ID_OFFSET: usize = 0;
pub const POOLS_LOG_TIME_OFFSET: usize = 8;
pub const POOLS_LOG_EVENT_TYPE_OFFSET: usize = 16;
pub const POOLS_LOG_FRACTIONS_OFFSET: usize = 20;
pub const POOLS_LOG_TASK_ID_OFFSET: usize = 24;
pub const POOLS_LOG_POOL_ID_OFFSET: usize = 28;
pub const POOLS_LOG_INSTR_ID_OFFSET: usize = 32;
pub const POOLS_LOG_COUNTER_OFFSET: usize = 36;
pub const POOLS_LOG_PUBKEY_OFFSET: usize = 44;
pub const POOLS_LOG_COST_OFFSET: usize = 76;
pub const POOLS_LOG_PX_OFFSET: usize = 84;
pub const POOLS_LOG_QTY_OFFSET: usize = 844;
pub const POOLS_LOG_VANILLA_MEMO_OFFSET: usize = 1224;
pub const POOLS_LOG_VANILLA_COST_OFFSET: usize = 1273;
pub const POOLS_LOG_SIZE: usize = 1305;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PoolsLogV8 {
    // 8 bytes, POOLS_LOG_ID_OFFSET
    pub id: [u8; 8],
    // 8 bytes, POOLS_LOG_TIME_OFFSET
    pub time: [u8; 8],
    // 4 bytes, POOLS_LOG_EVENT_TYPE_OFFSET
    pub event_type: [u8; 4],
    // 4 bytes, POOLS_LOG_FRACTIONS_OFFSET
    pub fractions: [u8; 4],
    // 4 bytes, POOLS_LOG_TASK_ID_OFFSET
    pub task_id: [u8; 4],
    // 4 bytes, POOLS_LOG_POOL_ID_OFFSET
    pub pool_id: [u8; 4],
    // 4 bytes, POOLS_LOG_INSTR_ID_OFFSET
    pub instr_id: [u8; 4],
    // 8 bytes, POOLS_LOG_COUNTER_OFFSET
    pub counter: [u8; 8],
    // 32 bytes, POOLS_LOG_PUBKEY_OFFSET
    pub pubkey: Pubkey,
    // 8 bytes, POOLS_LOG_COST_OFFSET
    pub cost: [u8; 8],
    // 760 bytes, POOLS_LOG_PX_OFFSET
    pub px: [[u8; 8]; BUCKETS_COUNT],
    // 380 bytes, POOLS_LOG_QTY_OFFSET
    pub qty: [[u8; 4]; BUCKETS_COUNT],
    // 49 bytes, POOLS_LOG_VANILLA_MEMO_OFFSET
    pub vanilla_memo: [u8; VANILLA_MEMO_SIZE],
    // 32 bytes, POOLS_LOG_VANILLA_COST_OFFSET
    pub vanilla_cost: [[u8; 8]; VANILLA_COST_SIZE],
}

impl PoolsLogV8 {
    #[inline(always)]
    pub fn get_id(&self) -> i64 {
        i64::from_ne_bytes(self.id)
    }

    #[inline(always)]
    pub fn set_id(&mut self, value: i64) {
        self.id = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_time(&self) -> i64 {
        i64::from_ne_bytes(self.time)
    }

    #[inline(always)]
    pub fn set_time(&mut self, value: i64) {
        self.time = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_event_type(&self) -> u32 {
        u32::from_ne_bytes(self.event_type)
    }

    #[inline(always)]
    pub fn set_event_type(&mut self, value: u32) {
        self.event_type = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_fractions(&self) -> u32 {
        u32::from_ne_bytes(self.fractions)
    }

    #[inline(always)]
    pub fn set_fractions(&mut self, value: u32) {
        self.fractions = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_task_id(&self) -> u32 {
        u32::from_ne_bytes(self.task_id)
    }

    #[inline(always)]
    pub fn set_task_id(&mut self, value: u32) {
        self.task_id = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_pool_id(&self) -> u32 {
        u32::from_ne_bytes(self.pool_id)
    }

    #[inline(always)]
    pub fn set_pool_id(&mut self, value: u32) {
        self.pool_id = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_instr_id(&self) -> u32 {
        u32::from_ne_bytes(self.instr_id)
    }

    #[inline(always)]
    pub fn set_instr_id(&mut self, value: u32) {
        self.instr_id = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_counter(&self) -> i64 {
        i64::from_ne_bytes(self.counter)
    }

    #[inline(always)]
    pub fn set_counter(&mut self, value: i64) {
        self.counter = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_cost(&self) -> i64 {
        i64::from_ne_bytes(self.cost)
    }

    #[inline(always)]
    pub fn set_cost(&mut self, value: i64) {
        self.cost = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_px(&self, index: usize) -> i64 {
        i64::from_ne_bytes(self.px[index])
    }

    #[inline(always)]
    pub fn set_px(&mut self, index: usize, value: i64) {
        self.px[index] = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn set_qty(&mut self, index: usize, value: i32) {
        self.qty[index] = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_qty(&self, index: usize) -> i32 {
        i32::from_ne_bytes(self.qty[index])
    }

    #[inline(always)]
    pub fn get_vanilla_cost(&self, index: usize) -> i64 {
        i64::from_ne_bytes(self.vanilla_cost[index])
    }

    #[inline(always)]
    pub fn set_vanilla_cost(&mut self, index: usize, value: i64) {
        self.vanilla_cost[index] = value.to_ne_bytes();
    }
}

#[cfg(test)]
impl Default for PoolsLogV8 {
    fn default() -> Self {
        Self {
            id: [0; 8],
            time: [0; 8],
            event_type: [0; 4],
            fractions: [0; 4],
            task_id: [0; 4],
            pool_id: [0; 4],
            instr_id: [0; 4],
            counter: [0; 8],
            pubkey: Pubkey::default(),
            cost: [0; 8],
            px: [[0; 8]; BUCKETS_COUNT],
            qty: [[0; 4]; BUCKETS_COUNT],
            vanilla_memo: [0; VANILLA_MEMO_SIZE],
            vanilla_cost: [[0; 8]; VANILLA_COST_SIZE],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use super::*;

    #[test]
    fn test_pools_log_offsets() {
        let log = PoolsLogV8::default();
        let base_ptr = &log as *const _ as usize;

        // checking fields size and offset
        assert_eq!(
            &log.id as *const _ as usize - base_ptr,
            POOLS_LOG_ID_OFFSET
        );
        assert_eq!(
            &log.time as *const _ as usize - base_ptr,
            POOLS_LOG_TIME_OFFSET
        );
        assert_eq!(
            &log.event_type as *const _ as usize - base_ptr,
            POOLS_LOG_EVENT_TYPE_OFFSET
        );
        assert_eq!(
            &log.fractions as *const _ as usize - base_ptr,
            POOLS_LOG_FRACTIONS_OFFSET
        );
        assert_eq!(
            &log.task_id as *const _ as usize - base_ptr,
            POOLS_LOG_TASK_ID_OFFSET
        );
        assert_eq!(
            &log.pool_id as *const _ as usize - base_ptr,
            POOLS_LOG_POOL_ID_OFFSET
        );
        assert_eq!(
            &log.instr_id as *const _ as usize - base_ptr,
            POOLS_LOG_INSTR_ID_OFFSET
        );
        assert_eq!(
            &log.counter as *const _ as usize - base_ptr,
            POOLS_LOG_COUNTER_OFFSET
        );
        assert_eq!(
            &log.pubkey as *const _ as usize - base_ptr,
            POOLS_LOG_PUBKEY_OFFSET
        );
        assert_eq!(
            &log.cost as *const _ as usize - base_ptr,
            POOLS_LOG_COST_OFFSET
        );
        assert_eq!(
            &log.px as *const _ as usize - base_ptr,
            POOLS_LOG_PX_OFFSET
        );
        assert_eq!(
            &log.qty as *const _ as usize - base_ptr,
            POOLS_LOG_QTY_OFFSET
        );
        assert_eq!(
            &log.vanilla_memo as *const _ as usize - base_ptr,
            POOLS_LOG_VANILLA_MEMO_OFFSET
        );
        assert_eq!(
            &log.vanilla_cost as *const _ as usize - base_ptr,
            POOLS_LOG_VANILLA_COST_OFFSET
        );

        // checking total size
        assert_eq!(mem::size_of::<PoolsLogV8>(), POOLS_LOG_SIZE);
    }
}