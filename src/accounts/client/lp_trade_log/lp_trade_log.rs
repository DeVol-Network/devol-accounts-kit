pub const LP_TRADE_LOG_ID_OFFSET: usize = 0;
pub const LP_TRADE_LOG_TIME_OFFSET: usize = 8;
pub const LP_TRADE_LOG_WORKER_ID_OFFSET: usize = 16;
pub const LP_TRADE_LOG_TASK_ID_OFFSET: usize = 20;
pub const LP_TRADE_LOG_COUNTER_OFFSET: usize = 24;
pub const LP_TRADE_LOG_COST_OFFSET: usize = 32;
pub const LP_TRADE_LOG_PS_OFFSET: usize = 40;
pub const LP_TRADE_LOG_PS_PX_OFFSET: usize = 44;
pub const LP_TRADE_LOG_SIZE: usize = 52;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LpTradeLog {
    // 8 bytes, LP_TRADE_LOG_ID_OFFSET
    pub id: [u8; 8],
    // 8 bytes, LP_TRADE_LOG_TIME_OFFSET
    pub time: [u8; 8],
    // 4 bytes, LP_TRADE_LOG_WORKER_ID_OFFSET
    pub worker_id: u32,
    // 4 bytes, LP_TRADE_LOG_TASK_ID_OFFSET
    pub task_id: u32,
    // 8 bytes, LP_TRADE_LOG_COUNTER_OFFSET
    pub counter: [u8; 8],
    // 8 bytes, LP_TRADE_LOG_COST_OFFSET
    pub cost: [u8; 8],
    // 4 bytes, LP_TRADE_LOG_PS_OFFSET
    pub ps: i32,
    // 8 bytes, LP_TRADE_LOG_PS_PX_OFFSET
    pub ps_px: [u8; 8],
}

impl LpTradeLog {
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
    pub fn get_ps_px(&self) -> i64 {
        i64::from_ne_bytes(self.ps_px)
    }

    #[inline(always)]
    pub fn set_ps_px(&mut self, value: i64) {
        self.ps_px = value.to_ne_bytes();
    }
}

impl Default for LpTradeLog {
    fn default() -> Self {
        Self {
            id: [0; 8],
            time: [0; 8],
            worker_id: 0,
            task_id: 0,
            counter: [0; 8],
            cost: [0; 8],
            ps: 0,
            ps_px: [0; 8],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use super::*;

    #[test]
    fn test_lp_trade_log_offsets() {
        let log = LpTradeLog::default();
        let base_ptr = &log as *const _ as usize;

        // checking fields size and offset
        assert_eq!(
            &log.id as *const _ as usize - base_ptr,
            LP_TRADE_LOG_ID_OFFSET
        );
        assert_eq!(
            &log.time as *const _ as usize - base_ptr,
            LP_TRADE_LOG_TIME_OFFSET
        );
        assert_eq!(
            &log.worker_id as *const _ as usize - base_ptr,
            LP_TRADE_LOG_WORKER_ID_OFFSET
        );
        assert_eq!(
            &log.task_id as *const _ as usize - base_ptr,
            LP_TRADE_LOG_TASK_ID_OFFSET
        );
        assert_eq!(
            &log.counter as *const _ as usize - base_ptr,
            LP_TRADE_LOG_COUNTER_OFFSET
        );
        assert_eq!(
            &log.cost as *const _ as usize - base_ptr,
            LP_TRADE_LOG_COST_OFFSET
        );
        assert_eq!(
            &log.ps as *const _ as usize - base_ptr,
            LP_TRADE_LOG_PS_OFFSET
        );
        assert_eq!(
            &log.ps_px as *const _ as usize - base_ptr,
            LP_TRADE_LOG_PS_PX_OFFSET
        );

        // checking total size
        assert_eq!(mem::size_of::<LpTradeLog>(), LP_TRADE_LOG_SIZE);
    }
}