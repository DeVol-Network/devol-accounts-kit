use crate::constants::{BUCKETS_COUNT, VANILLA_COST_SIZE, VANILLA_MEMO_SIZE};

pub const TRADE_LOG_ID_OFFSET: usize = 0;
pub const TRADE_LOG_TIME_OFFSET: usize = 8;
pub const TRADE_LOG_FRACTIONS_OFFSET: usize = 16;
pub const TRADE_LOG_WORKER_ID_OFFSET: usize = 20;
pub const TRADE_LOG_TASK_ID_OFFSET: usize = 24;
pub const TRADE_LOG_POOL_ID_OFFSET: usize = 28;
pub const TRADE_LOG_INSTR_ID_OFFSET: usize = 32;
pub const TRADE_LOG_COUNTER_OFFSET: usize = 36;
pub const TRADE_LOG_COST_OFFSET: usize = 44;
pub const TRADE_LOG_PX_OFFSET: usize = 52;
pub const TRADE_LOG_QTY_OFFSET: usize = 812;
pub const TRADE_LOG_VANILLA_MEMO_OFFSET: usize = 1192;
pub const TRADE_LOG_VANILLA_COST_OFFSET: usize = 1241;
pub const TRADE_LOG_SIZE: usize = 1273;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TradeLog {
    // 8 bytes, TRADE_LOG_ID_OFFSET
    pub id: [u8; 8],
    // 8 bytes, TRADE_LOG_TIME_OFFSET
    pub time: [u8; 8],
    // 4 bytes, TRADE_LOG_FRACTIONS_OFFSET
    pub fractions: [u8; 4],
    // 4 bytes, TRADE_LOG_WORKER_ID_OFFSET
    pub worker_id: [u8; 4],
    // 4 bytes, TRADE_LOG_TASK_ID_OFFSET
    pub task_id: [u8; 4],
    // 4 bytes, TRADE_LOG_POOL_ID_OFFSET
    pub pool_id: [u8; 4],
    // 4 bytes, TRADE_LOG_INSTR_ID_OFFSET
    pub instr_id: [u8; 4],
    // 8 bytes, TRADE_LOG_COUNTER_OFFSET
    pub counter: [u8; 8],
    // 8 bytes, TRADE_LOG_COST_OFFSET
    pub cost: [u8; 8],
    // 760 bytes, TRADE_LOG_PX_OFFSET
    pub px: [[u8; 8]; BUCKETS_COUNT],
    // 380 bytes, TRADE_LOG_QTY_OFFSET
    pub qty: [[u8; 4]; BUCKETS_COUNT],
    // 49 bytes, TRADE_LOG_VANILLA_MEMO_OFFSET
    pub vanilla_memo: [u8; VANILLA_MEMO_SIZE],
    // 32 bytes, TRADE_LOG_VANILLA_COST_OFFSET
    pub vanilla_cost: [[u8; 8]; VANILLA_COST_SIZE],
}

impl TradeLog {
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
    pub fn get_fractions(&self) -> u32 {
        u32::from_ne_bytes(self.fractions)
    }

    #[inline(always)]
    pub fn set_fractions(&mut self, value: u32) {
        self.fractions = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_worker_id(&self) -> u32 {
        u32::from_ne_bytes(self.worker_id)
    }

    #[inline(always)]
    pub fn set_worker_id(&mut self, value: u32) {
        self.worker_id = value.to_ne_bytes();
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
    pub fn get_qty(&self, index: usize) -> i32 {
        i32::from_ne_bytes(self.qty[index])
    }

    #[inline(always)]
    pub fn set_qty(&mut self, index: usize, value: i32) {
        self.qty[index] = value.to_ne_bytes();
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
impl Default for TradeLog {
    fn default() -> Self {
        Self {
            id: [0; 8],
            time: [0; 8],
            fractions: [0; 4],
            worker_id: [0; 4],
            task_id: [0; 4],
            pool_id: [0; 4],
            instr_id: [0; 4],
            counter: [0; 8],
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
    use super::*;

    #[test]
    fn test_trade_log_offsets() {
        let log = TradeLog::default();
        let base_ptr = &log as *const _ as usize;

        // checking fields size and offset
        assert_eq!(
            &log.id as *const _ as usize - base_ptr,
            TRADE_LOG_ID_OFFSET
        );
        assert_eq!(
            &log.time as *const _ as usize - base_ptr,
            TRADE_LOG_TIME_OFFSET
        );
        assert_eq!(
            &log.fractions as *const _ as usize - base_ptr,
            TRADE_LOG_FRACTIONS_OFFSET
        );
        assert_eq!(
            &log.worker_id as *const _ as usize - base_ptr,
            TRADE_LOG_WORKER_ID_OFFSET
        );
        assert_eq!(
            &log.task_id as *const _ as usize - base_ptr,
            TRADE_LOG_TASK_ID_OFFSET
        );
        assert_eq!(
            &log.pool_id as *const _ as usize - base_ptr,
            TRADE_LOG_POOL_ID_OFFSET
        );
        assert_eq!(
            &log.instr_id as *const _ as usize - base_ptr,
            TRADE_LOG_INSTR_ID_OFFSET
        );
        assert_eq!(
            &log.counter as *const _ as usize - base_ptr,
            TRADE_LOG_COUNTER_OFFSET
        );
        assert_eq!(
            &log.cost as *const _ as usize - base_ptr,
            TRADE_LOG_COST_OFFSET
        );
        assert_eq!(
            &log.px as *const _ as usize - base_ptr,
            TRADE_LOG_PX_OFFSET
        );
        assert_eq!(
            &log.qty as *const _ as usize - base_ptr,
            TRADE_LOG_QTY_OFFSET
        );
        assert_eq!(
            &log.vanilla_memo as *const _ as usize - base_ptr,
            TRADE_LOG_VANILLA_MEMO_OFFSET
        );
        assert_eq!(
            &log.vanilla_cost as *const _ as usize - base_ptr,
            TRADE_LOG_VANILLA_COST_OFFSET
        );

        // checking total size
        assert_eq!(std::mem::size_of::<TradeLog>(), TRADE_LOG_SIZE);
    }
}
