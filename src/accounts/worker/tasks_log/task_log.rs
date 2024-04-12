use solana_program::pubkey::Pubkey;

pub const TASKS_LOG_ID_OFFSET: usize = 0;
pub const TASKS_LOG_TIME_OFFSET: usize = 8;
pub const TASKS_LOG_EVENT_TYPE_OFFSET: usize = 16;
pub const TASKS_LOG_TASK_ID_OFFSET: usize = 20;
pub const TASKS_LOG_POOL_ID_OFFSET: usize = 24;
pub const TASKS_LOG_INSTR_ID_OFFSET: usize = 28;
pub const TASKS_LOG_COUNTER_OFFSET: usize = 32;
pub const TASKS_LOG_PUBKEY_OFFSET: usize = 40;
pub const TASKS_LOG_COST_OFFSET: usize = 72;
pub const TASKS_LOG_TRADE_QTY_OFFSET: usize = 80;
pub const TASKS_LOG_TRADE_PX_OFFSET: usize = 84;
pub const TASKS_LOG_PS_OFFSET: usize = 92;
pub const TASKS_LOG_PS_PX_OFFSET: usize = 96;
pub const TASKS_LOG_SIZE: usize = 104;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TasksLog {
    // 8 bytes, TASKS_LOG_ID_OFFSET
    pub id: [u8; 8],
    // 8 bytes, TASKS_LOG_TIME_OFFSET
    pub time: [u8; 8],
    // 4 bytes, TASKS_LOG_EVENT_TYPE_OFFSET
    pub event_type: u32,
    // 4 bytes, TASKS_LOG_TASK_ID_OFFSET
    pub task_id: u32,
    // 4 bytes, TASKS_LOG_POOL_ID_OFFSET
    pub pool_id: u32,
    // 4 bytes, TASKS_LOG_INSTR_ID_OFFSET
    pub instr_id: u32,
    // 8 bytes, TASKS_LOG_COUNTER_OFFSET
    pub counter: [u8; 8],
    // 32 bytes, TASKS_LOG_PUBKEY_OFFSET
    pub pubkey: Pubkey,
    // 8 bytes, TASKS_LOG_COST_OFFSET
    pub cost: [u8; 8],
    // 4 bytes, TASKS_LOG_TRADE_QTY_OFFSET
    pub trade_qty: i32,
    // 8 bytes, TASKS_LOG_TRADE_PX_OFFSET
    pub trade_px: [u8; 8],
    // 4 bytes, TASKS_LOG_PS_OFFSET
    pub ps: u32,
    // 8 bytes, TASKS_LOG_PS_PX_OFFSET
    pub ps_px: [u8; 8],
}

impl TasksLog {
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
    pub fn get_trade_px(&self) -> i64 {
        i64::from_ne_bytes(self.trade_px)
    }

    #[inline(always)]
    pub fn set_trade_px(&mut self, value: i64) {
        self.trade_px = value.to_ne_bytes();
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

#[cfg(test)]
impl Default for TasksLog {
    fn default() -> Self {
        Self {
            id: [0; 8],
            time: [0; 8],
            event_type: 0,
            task_id: 0,
            pool_id: 0,
            instr_id: 0,
            counter: [0; 8],
            pubkey: Pubkey::default(),
            cost: [0; 8],
            trade_qty: 0,
            trade_px: [0; 8],
            ps: 0,
            ps_px: [0; 8],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tasks_log_offsets() {
        let log = TasksLog::default();
        let base_ptr = &log as *const _ as usize;

        // checking fields size and offset
        assert_eq!(
            &log.id as *const _ as usize - base_ptr,
            TASKS_LOG_ID_OFFSET
        );
        assert_eq!(
            &log.time as *const _ as usize - base_ptr,
            TASKS_LOG_TIME_OFFSET
        );
        assert_eq!(
            &log.event_type as *const _ as usize - base_ptr,
            TASKS_LOG_EVENT_TYPE_OFFSET
        );
        assert_eq!(
            &log.task_id as *const _ as usize - base_ptr,
            TASKS_LOG_TASK_ID_OFFSET
        );
        assert_eq!(
            &log.pool_id as *const _ as usize - base_ptr,
            TASKS_LOG_POOL_ID_OFFSET
        );
        assert_eq!(
            &log.instr_id as *const _ as usize - base_ptr,
            TASKS_LOG_INSTR_ID_OFFSET
        );
        assert_eq!(
            &log.counter as *const _ as usize - base_ptr,
            TASKS_LOG_COUNTER_OFFSET
        );
        assert_eq!(
            &log.pubkey as *const _ as usize - base_ptr,
            TASKS_LOG_PUBKEY_OFFSET
        );
        assert_eq!(
            &log.cost as *const _ as usize - base_ptr,
            TASKS_LOG_COST_OFFSET
        );
        assert_eq!(
            &log.trade_qty as *const _ as usize - base_ptr,
            TASKS_LOG_TRADE_QTY_OFFSET
        );
        assert_eq!(
            &log.trade_px as *const _ as usize - base_ptr,
            TASKS_LOG_TRADE_PX_OFFSET
        );
        assert_eq!(
            &log.ps as *const _ as usize - base_ptr,
            TASKS_LOG_PS_OFFSET
        );
        assert_eq!(
            &log.ps_px as *const _ as usize - base_ptr,
            TASKS_LOG_PS_PX_OFFSET
        );

        // checking total size
        assert_eq!(std::mem::size_of::<TasksLog>(), TASKS_LOG_SIZE);
    }
}
