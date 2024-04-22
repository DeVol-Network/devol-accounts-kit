use std::error::Error;
use crate::instructions::devol_instruction_data::DevolInstructionData;
use crate::instructions::instructions::Instructions;


pub const INSTRUCTION_WITHDRAW_TOKEN_SIZE: usize = 16;

#[repr(C)]
pub struct InstructionWithdrawToken {
    pub cmd: u8,
    pub version: u8,
    pub reserved: [u8; 2],
    pub mint_id: u32,
    pub amount: u64,
}

pub struct WithdrawTokenParams {
    pub mint_id: u32,
    pub amount: u64,
}

impl<'a> DevolInstructionData<'a> for InstructionWithdrawToken {
    type DvlInstrParams = WithdrawTokenParams;

    fn new(params: Self::DvlInstrParams) -> Result<Box<InstructionWithdrawToken>, Box<dyn Error>> {
        Ok(Box::new(InstructionWithdrawToken {
            cmd: Instructions::WithdrawToken as u8,
            version: INSTRUCTION_VERSION,
            reserved: [0; 2],
            mint_id: params.mint_id,
            amount: params.amount,
        }))
    }
}

const INSTRUCTION_VERSION: u8 = 1;



#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use crate::instructions::devol_instruction_data::DvlInstruction;

    pub const INSTR_WITHDRAW_TOKEN_CMD_OFFSET: usize = 0;
    pub const INSTR_WITHDRAW_TOKEN_VERSION_OFFSET: usize = 1;
    pub const INSTR_WITHDRAW_TOKEN_RESERVED_OFFSET: usize = 2;
    pub const INSTR_WITHDRAW_TOKEN_MINT_ID_OFFSET: usize = 4;
    pub const INSTR_WITHDRAW_AMOUNT_ID_OFFSET: usize = 8;

    #[test]
    fn test_instruction_data_offsets() {

        let withdraw_token_params = WithdrawTokenParams {
            mint_id: 1,
            amount: 2,
        };

        let data = DvlInstruction::new::<InstructionWithdrawToken>(withdraw_token_params).unwrap();

        let base_ptr = &*data as *const _ as usize;

        assert_eq!(&data.cmd as *const _ as usize - base_ptr, INSTR_WITHDRAW_TOKEN_CMD_OFFSET);
        assert_eq!(&data.version as *const _ as usize - base_ptr, INSTR_WITHDRAW_TOKEN_VERSION_OFFSET);
        assert_eq!(&data.reserved as *const _ as usize - base_ptr, INSTR_WITHDRAW_TOKEN_RESERVED_OFFSET);
        assert_eq!(&data.mint_id as *const _ as usize - base_ptr, INSTR_WITHDRAW_TOKEN_MINT_ID_OFFSET);
        assert_eq!(&data.amount as *const _ as usize - base_ptr, INSTR_WITHDRAW_AMOUNT_ID_OFFSET);

        assert_eq!(mem::size_of::<InstructionWithdrawToken>(), INSTRUCTION_WITHDRAW_TOKEN_SIZE);
        assert_eq!(data.mint_id, 1);
        assert_eq!(data.amount, 2);
    }
}