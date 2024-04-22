use std::error::Error;
use crate::instructions::devol_instruction_data::DevolInstructionData;
use crate::instructions::instructions::Instructions;


pub const INSTRUCTION_LP_TRADE_SIZE: usize = 8;

#[repr(C)]
pub struct InstructionLpTrade {
    pub cmd: u8,
    pub version: u8,
    pub reserved: [u8; 2],
    pub trade_qty: u32,
}

pub struct LpTradeParams {
    pub trade_qty: u32,
}

impl<'a> DevolInstructionData<'a> for InstructionLpTrade {
    type DvlInstrParams = LpTradeParams;

    fn new(params: Self::DvlInstrParams) -> Result<Box<InstructionLpTrade>, Box<dyn Error>> {
        Ok(Box::new(InstructionLpTrade {
            cmd: Instructions::FinPool as u8,
            version: INSTRUCTION_VERSION,
            reserved: [0; 2],
            trade_qty: params.trade_qty

        }))
    }
}

const INSTRUCTION_VERSION: u8 = 1;



#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use crate::instructions::devol_instruction_data::DvlInstruction;

    pub const INSTR_LP_TRADE_CMD_OFFSET: usize = 0;
    pub const INSTR_LP_TRADE_VERSION_OFFSET: usize = 1;
    pub const INSTR_LP_TRADE_RESERVED_OFFSET: usize = 2;
    pub const INSTR_LP_TRADE_QTY_OFFSET: usize = 4;


    #[test]
    fn test_instruction_data_offsets() {

        let lp_trade_params = LpTradeParams {
            trade_qty: 1
        };

        let data = DvlInstruction::new::<InstructionLpTrade>(lp_trade_params).unwrap();

        let base_ptr = &*data as *const _ as usize;

        assert_eq!(&data.cmd as *const _ as usize - base_ptr, INSTR_LP_TRADE_CMD_OFFSET);
        assert_eq!(&data.version as *const _ as usize - base_ptr, INSTR_LP_TRADE_VERSION_OFFSET);
        assert_eq!(&data.reserved as *const _ as usize - base_ptr, INSTR_LP_TRADE_RESERVED_OFFSET);
        assert_eq!(&data.trade_qty as *const _ as usize - base_ptr, INSTR_LP_TRADE_QTY_OFFSET);

        assert_eq!(mem::size_of::<InstructionLpTrade>(), INSTRUCTION_LP_TRADE_SIZE);
        assert_eq!(data.trade_qty, 1);
    }
}