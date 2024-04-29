pub const INSTRUCTION_PAYOFF_SIZE: usize = 12;

#[repr(C)]
pub struct InstructionPayoff {
    pub cmd: u8,
    pub version: u8,
    pub reserved: [u8; 2],
    pub worker_id: u32,
    pub pool_id: u32,
}


#[cfg(test)]
mod tests {
    use std::mem;
    use crate::instructions_data::constructors::payoff::PayoffParams;
    use crate::instructions_data::dvl_instruction_data::DvlInstruction;
    use crate::instructions_data::payoff::{INSTRUCTION_PAYOFF_SIZE, InstructionPayoff};

    pub const INSTR_PAYOFF_CMD_OFFSET: usize = 0;
    pub const INSTR_PAYOFF_VERSION_OFFSET: usize = 1;
    pub const INSTR_PAYOFF_RESERVED_OFFSET: usize = 2;
    pub const INSTR_PAYOFF_WORKER_ID_OFFSET: usize = 4;
    pub const INSTR_PAYOFF_POOL_ID_OFFSET: usize = 8;

    #[test]
    fn test_instruction_data_offsets() {
        let payoff_params = PayoffParams {
            worker_id: 1,
            pool_id: 1,
        };

        let data = DvlInstruction::new::<InstructionPayoff>(payoff_params).unwrap();

        let base_ptr = &*data as *const _ as usize;

        assert_eq!(&data.cmd as *const _ as usize - base_ptr, INSTR_PAYOFF_CMD_OFFSET);
        assert_eq!(&data.version as *const _ as usize - base_ptr, INSTR_PAYOFF_VERSION_OFFSET);
        assert_eq!(&data.reserved as *const _ as usize - base_ptr, INSTR_PAYOFF_RESERVED_OFFSET);
        assert_eq!(&data.worker_id as *const _ as usize - base_ptr, INSTR_PAYOFF_WORKER_ID_OFFSET);
        assert_eq!(&data.pool_id as *const _ as usize - base_ptr, INSTR_PAYOFF_POOL_ID_OFFSET);

        assert_eq!(mem::size_of::<InstructionPayoff>(), INSTRUCTION_PAYOFF_SIZE);
    }
}