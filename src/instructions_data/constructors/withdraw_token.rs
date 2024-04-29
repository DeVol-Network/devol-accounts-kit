use std::error::Error;
use crate::instructions_data::dvl_instruction_data::DvlInstructionData;
use crate::instructions_data::instructions::Instructions;
use crate::instructions_data::withdraw_token::{INSTRUCTION_WITHDRAW_TOKEN_SIZE, INSTRUCTION_WITHDRAW_TOKEN_VERSION, InstructionWithdrawToken};

pub struct WithdrawTokenParams {
    pub mint_id: u32,
    pub amount: u64,
}

impl<'a> DvlInstructionData<'a> for InstructionWithdrawToken {
    #[inline(always)]
    fn expected_size() -> usize {INSTRUCTION_WITHDRAW_TOKEN_SIZE}
    #[inline(always)]
    fn expected_version() -> u8 {INSTRUCTION_WITHDRAW_TOKEN_VERSION}

    type DvlInstrParams = WithdrawTokenParams;

    fn new(params: Self::DvlInstrParams) -> Result<Box<InstructionWithdrawToken>, Box<dyn Error>> {
        Ok(Box::new(InstructionWithdrawToken {
            cmd: Instructions::WithdrawToken as u8,
            version: INSTRUCTION_WITHDRAW_TOKEN_VERSION,
            reserved: [0; 2],
            mint_id: params.mint_id,
            amount: params.amount,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions_data::dvl_instruction_data::DvlInstruction;

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
        assert_eq!(data.version, INSTRUCTION_WITHDRAW_TOKEN_VERSION);
        assert_eq!(data.mint_id, TEST_MINT_ID);
        assert_eq!(data.amount, TEST_AMOUNT);
    }
}