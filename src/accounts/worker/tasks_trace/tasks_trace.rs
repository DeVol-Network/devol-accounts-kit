pub const TASKS_TRACE_INSTR_ID_OFFSET: usize = 0;
pub const TASKS_TRACE_FIRST_TIME_OFFSET: usize = 4;
pub const TASKS_TRACE_DURATION_OFFSET: usize = 12;
pub const TASKS_TRACE_INIT_PX_OFFSET: usize = 16;
pub const TASKS_TRACE_UPDATE_TIME_OFFSET: usize = 24;
pub const TASKS_TRACE_PS_OFFSET: usize = 32;
pub const TASKS_TRACE_PS_PX_OFFSET: usize = 36;
pub const TASKS_TRACE_CANCEL_TIME_OFFSET: usize = 44;
pub const TASKS_TRACE_PAYOFF_TIME_OFFSET: usize = 52;
pub const TASKS_TRACE_SIZE: usize = 60;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TasksTrace {
    // 4 bytes, TASKS_TRACE_INSTR_ID_OFFSET
    pub instr_id: u32,
    // 8 bytes, TASKS_TRACE_FIRST_TIME_OFFSET
    pub first_time: [u8; 8],
    // 4 bytes, TASKS_TRACE_DURATION_OFFSET
    pub duration: u32,
    // 8 bytes, TASKS_TRACE_INIT_PX_OFFSET
    pub init_px: [u8; 8],
    // 8 bytes, TASKS_TRACE_UPDATE_TIME_OFFSET
    pub update_time: [u8; 8],
    // 4 bytes, TASKS_TRACE_PS_OFFSET
    pub ps: u32,
    // 8 bytes, TASKS_TRACE_PS_PX_OFFSET
    pub ps_px: [u8; 8],
    // 8 bytes, TASKS_TRACE_CANCEL_TIME_OFFSET
    pub cancel_time: [u8; 8],
    // 8 bytes, TASKS_TRACE_PAYOFF_TIME_OFFSET
    pub payoff_time: [u8; 8],
}

impl TasksTrace {
    #[inline(always)]
    pub fn get_first_time(&self) -> i64 {
        i64::from_ne_bytes(self.first_time)
    }

    #[inline(always)]
    pub fn set_first_time(&mut self, value: i64) {
        self.first_time = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_init_px(&self) -> i64 {
        i64::from_ne_bytes(self.init_px)
    }

    #[inline(always)]
    pub fn set_init_px(&mut self, value: i64) {
        self.init_px = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_update_time(&self) -> i64 {
        i64::from_ne_bytes(self.update_time)
    }

    #[inline(always)]
    pub fn set_update_time(&mut self, value: i64) {
        self.update_time = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_ps_px(&self) -> i64 {
        i64::from_ne_bytes(self.ps_px)
    }

    #[inline(always)]
    pub fn set_ps_px(&mut self, value: i64) {
        self.ps_px = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_cancel_time(&self) -> i64 {
        i64::from_ne_bytes(self.cancel_time)
    }

    #[inline(always)]
    pub fn set_cancel_time(&mut self, value: i64) {
        self.cancel_time = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_payoff_time(&self) -> i64 {
        i64::from_ne_bytes(self.payoff_time)
    }

    #[inline(always)]
    pub fn set_payoff_time(&mut self, value: i64) {
        self.payoff_time = value.to_ne_bytes();
    }
}

impl Default for TasksTrace {
    fn default() -> Self {
        Self {
            instr_id: 0,
            first_time: [0; 8],
            duration: 0,
            init_px: [0; 8],
            update_time: [0; 8],
            ps: 0,
            ps_px: [0; 8],
            cancel_time: [0; 8],
            payoff_time: [0; 8],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tasks_trace_offsets() {
        let trace = TasksTrace::default();
        let base_ptr = &trace as *const _ as usize;

        // checking fields size and offset
        assert_eq!(
            unsafe { &trace.instr_id as *const _ as usize } - base_ptr,
            TASKS_TRACE_INSTR_ID_OFFSET
        );
        assert_eq!(
            unsafe { &trace.first_time as *const _ as usize } - base_ptr,
            TASKS_TRACE_FIRST_TIME_OFFSET
        );
        assert_eq!(
            unsafe { &trace.duration as *const _ as usize } - base_ptr,
            TASKS_TRACE_DURATION_OFFSET
        );
        assert_eq!(
            unsafe { &trace.init_px as *const _ as usize } - base_ptr,
            TASKS_TRACE_INIT_PX_OFFSET
        );
        assert_eq!(
            unsafe { &trace.update_time as *const _ as usize } - base_ptr,
            TASKS_TRACE_UPDATE_TIME_OFFSET
        );
        assert_eq!(
            unsafe { &trace.ps as *const _ as usize } - base_ptr,
            TASKS_TRACE_PS_OFFSET
        );
        assert_eq!(
            unsafe { &trace.ps_px as *const _ as usize } - base_ptr,
            TASKS_TRACE_PS_PX_OFFSET
        );
        assert_eq!(
            unsafe { &trace.cancel_time as *const _ as usize } - base_ptr,
            TASKS_TRACE_CANCEL_TIME_OFFSET
        );
        assert_eq!(
            unsafe { &trace.payoff_time as *const _ as usize } - base_ptr,
            TASKS_TRACE_PAYOFF_TIME_OFFSET
        );

        // checking total size
        assert_eq!(std::mem::size_of::<TasksTrace>(), TASKS_TRACE_SIZE);
    }
}
