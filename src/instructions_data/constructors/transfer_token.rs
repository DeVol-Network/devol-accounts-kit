use std::error::Error;
use crate::instructions_data::dvl_instruction_data::DvlInstructionData;
use crate::instructions_data::instructions::Instructions;
use crate::instructions_data::transfer_token::{INSTRUCTION_TRANSFER_TOKEN_SIZE, INSTRUCTION_TRANSFER_TOKEN_VERSION, InstructionTransferToken};

pub struct TransferTokenParams {
    pub mint_id: u32,
    pub amount: u64,
}

impl<'a> DvlInstructionData<'a> for InstructionTransferToken {
    #[inline(always)]
    fn expected_size() -> usize {INSTRUCTION_TRANSFER_TOKEN_SIZE}
    #[inline(always)]
    fn expected_version() -> u8 {INSTRUCTION_TRANSFER_TOKEN_VERSION}

    type DvlInstrParams = TransferTokenParams;

    fn new(params: Self::DvlInstrParams) -> Result<Box<InstructionTransferToken>, Box<dyn Error>> {
        Ok(Box::new(InstructionTransferToken {
            cmd: Instructions::TransferToken as u8,
            version: INSTRUCTION_TRANSFER_TOKEN_VERSION,
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
    fn test_instruction_transfer_token_params() {
        const TEST_MINT_ID: u32 = 1;
        const TEST_AMOUNT: u64 = 2;

        let withdraw_token_params = TransferTokenParams {
            mint_id: TEST_MINT_ID,
            amount: TEST_AMOUNT,
        };
        let data = DvlInstruction::new::<InstructionTransferToken>(withdraw_token_params).unwrap();
        assert_eq!(data.cmd, Instructions::TransferToken as u8);
        assert_eq!(data.version, INSTRUCTION_TRANSFER_TOKEN_VERSION);
        assert_eq!(data.mint_id, TEST_MINT_ID);
        assert_eq!(data.amount, TEST_AMOUNT);
    }
}