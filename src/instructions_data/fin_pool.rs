use crate::instructions_data::dvl_deserializable_instruction::DvlDeserializableInstruction;

pub const INSTRUCTION_FIN_POOL_SIZE: usize = 16;
pub const INSTRUCTION_FIN_POOL_VERSION: u8 = 2;

#[repr(C)]
pub struct InstructionFinPool {
    pub cmd: u8,
    pub version: u8,
    pub terminate: bool,
    pub reserved: [u8; 5],
    pub price: i64,
}

impl<'a> DvlDeserializableInstruction<'a> for InstructionFinPool {
    #[inline(always)]
    fn expected_size() -> usize { INSTRUCTION_FIN_POOL_SIZE }
    #[inline(always)]
    fn expected_version() -> u8 { INSTRUCTION_FIN_POOL_VERSION }
}

#[cfg(not(feature = "on-chain"))]
#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use crate::instructions_data::constructors::fin_pool::FinPoolParams;
    use crate::instructions_data::dvl_instruction_data::DvlInstruction;

    pub const INSTR_FIN_POOL_CMD_OFFSET: usize = 0;
    pub const INSTR_FIN_POOL_VERSION_OFFSET: usize = 1;
    pub const INSTR_FIN_POOL_TERMINATE_OFFSET: usize = 2;
    pub const INSTR_FIN_POOL_RESERVED_OFFSET: usize = 3;
    pub const INSTR_FIN_POOL_PRICE_OFFSET: usize = 8;


    #[test]
    fn test_instruction_data_offsets() {
        let fin_pool_params = FinPoolParams {
            price: 1.,
            terminate: true,
        };

        let data = DvlInstruction::new::<InstructionFinPool>(fin_pool_params).unwrap();

        let base_ptr = &*data as *const _ as usize;

        assert_eq!(&data.cmd as *const _ as usize - base_ptr, INSTR_FIN_POOL_CMD_OFFSET);
        assert_eq!(&data.version as *const _ as usize - base_ptr, INSTR_FIN_POOL_VERSION_OFFSET);
        assert_eq!(&data.reserved as *const _ as usize - base_ptr, INSTR_FIN_POOL_RESERVED_OFFSET);
        assert_eq!(&data.price as *const _ as usize - base_ptr, INSTR_FIN_POOL_PRICE_OFFSET);
        assert_eq!(&data.terminate as *const _ as usize - base_ptr, INSTR_FIN_POOL_TERMINATE_OFFSET);

        assert_eq!(mem::size_of::<InstructionFinPool>(), INSTRUCTION_FIN_POOL_SIZE);
    }
}