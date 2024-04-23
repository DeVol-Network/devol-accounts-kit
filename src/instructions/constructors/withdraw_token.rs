use std::error::Error;
use crate::instructions::devol_instruction_data::DevolInstructionData;
use crate::instructions::instructions::Instructions;
use crate::instructions::withdraw_token::{InstructionWithdrawToken};

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
    use crate::instructions::devol_instruction_data::DvlInstruction;

    #[test]
    fn test_instruction_withdraw_token_params() {
        const TEST_MINT_ID: u32 = 1;
        const TEST_AMOUNT: u64 = 2;

        let withdraw_token_params = WithdrawTokenParams {
            mint_id: TEST_MINT_ID,
            amount: TEST_AMOUNT,
        };
        let data = DvlInstruction::new::<InstructionWithdrawToken>(withdraw_token_params).unwrap();
        assert_eq!(data.cmd, Instructions::WithdrawToken as u8);
        assert_eq!(data.version, INSTRUCTION_VERSION);
        assert_eq!(data.mint_id, TEST_MINT_ID);
        assert_eq!(data.amount, TEST_AMOUNT);
    }
}