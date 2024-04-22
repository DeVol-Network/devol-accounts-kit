use std::error::Error;
use crate::instructions::devol_instruction_data::DevolInstructionData;
use crate::instructions::instructions::Instructions;


pub const INSTRUCTION_START_POOL_SIZE: usize = 24;

#[repr(C)]
pub struct InstructionStartPool {
    pub cmd: u8,
    pub version: u8,
    pub reserved: [u8; 6],
    pub price: u64,
    pub sigma: u64,
}

pub struct StartPoolParams {
    pub price: u64,
    pub sigma: u64,
}

impl<'a> DevolInstructionData<'a> for InstructionStartPool {
    type DvlInstrParams = StartPoolParams;

    fn new(params: Self::DvlInstrParams) -> Result<Box<InstructionStartPool>, Box<dyn Error>> {
        Ok(Box::new(InstructionStartPool {
            cmd: Instructions::StartPool as u8,
            version: INSTRUCTION_VERSION,
            reserved: [0; 6],
            price: params.price,
            sigma: params.sigma,

        }))
    }
}

const INSTRUCTION_VERSION: u8 = 1;



#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use crate::instructions::devol_instruction_data::DvlInstruction;

    pub const INSTR_START_POOL_CMD_OFFSET: usize = 0;
    pub const INSTR_START_POOL_VERSION_OFFSET: usize = 1;
    pub const INSTR_START_POOL_RESERVED_OFFSET: usize = 2;
    pub const INSTR_START_POOL_PRICE_OFFSET: usize = 8;
    pub const INSTR_START_POOL_SIGMA_OFFSET: usize = 16;



    #[test]
    fn test_instruction_data_offsets() {

        let start_pool_params = StartPoolParams {
            price: 1,
            sigma: 2,
        };

        let data = DvlInstruction::new::<InstructionStartPool>(start_pool_params).unwrap();

        let base_ptr = &*data as *const _ as usize;

        assert_eq!(&data.cmd as *const _ as usize - base_ptr, INSTR_START_POOL_CMD_OFFSET);
        assert_eq!(&data.version as *const _ as usize - base_ptr, INSTR_START_POOL_VERSION_OFFSET);
        assert_eq!(&data.reserved as *const _ as usize - base_ptr, INSTR_START_POOL_RESERVED_OFFSET);
        assert_eq!(&data.price as *const _ as usize - base_ptr, INSTR_START_POOL_PRICE_OFFSET);
        assert_eq!(&data.sigma as *const _ as usize - base_ptr, INSTR_START_POOL_SIGMA_OFFSET);

        assert_eq!(mem::size_of::<InstructionStartPool>(), INSTRUCTION_START_POOL_SIZE);
        assert_eq!(data.price, 1);
        assert_eq!(data.sigma, 2);
    }
}