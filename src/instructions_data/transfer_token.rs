pub const INSTRUCTION_TRANSFER_TOKEN_SIZE: usize = 16;
pub const INSTRUCTION_TRANSFER_TOKEN_VERSION: u8 = 2;

#[repr(C)]
pub struct InstructionTransferToken {
    pub cmd: u8,
    pub version: u8,
    pub reserved: [u8; 2],
    pub mint_id: u32,
    pub amount: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use crate::instructions_data::constructors::transfer_token::TransferTokenParams;
    use crate::instructions_data::dvl_instruction_data::DvlInstruction;

    pub const INSTR_TRANSFER_TOKEN_CMD_OFFSET: usize = 0;
    pub const INSTR_TRANSFER_TOKEN_VERSION_OFFSET: usize = 1;
    pub const INSTR_TRANSFER_TOKEN_RESERVED_OFFSET: usize = 2;
    pub const INSTR_TRANSFER_TOKEN_MINT_ID_OFFSET: usize = 4;
    pub const INSTR_TRANSFER_AMOUNT_ID_OFFSET: usize = 8;

    #[test]
    fn test_instruction_data_offsets() {
        let transfer_token_params = TransferTokenParams {
            mint_id: 1,
            amount: 2,
        };

        let data = DvlInstruction::new::<InstructionTransferToken>(transfer_token_params).unwrap();

        let base_ptr = &*data as *const _ as usize;

        assert_eq!(&data.cmd as *const _ as usize - base_ptr, INSTR_TRANSFER_TOKEN_CMD_OFFSET);
        assert_eq!(&data.version as *const _ as usize - base_ptr, INSTR_TRANSFER_TOKEN_VERSION_OFFSET);
        assert_eq!(&data.reserved as *const _ as usize - base_ptr, INSTR_TRANSFER_TOKEN_RESERVED_OFFSET);
        assert_eq!(&data.mint_id as *const _ as usize - base_ptr, INSTR_TRANSFER_TOKEN_MINT_ID_OFFSET);
        assert_eq!(&data.amount as *const _ as usize - base_ptr, INSTR_TRANSFER_AMOUNT_ID_OFFSET);

        assert_eq!(mem::size_of::<InstructionTransferToken>(), INSTRUCTION_TRANSFER_TOKEN_SIZE);
        assert_eq!(data.mint_id, 1);
        assert_eq!(data.amount, 2);
    }
}