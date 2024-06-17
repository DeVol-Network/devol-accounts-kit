use crate::constants::BUCKETS_COUNT;
use crate::instructions_data::common::INSTRUCTION_ACCOUNT_INFO_REF_SIZE;
use crate::utils::option_trade_basket_data::OptionTradeBasketData;
use crate::instructions_data::dvl_deserializable_instruction::DvlDeserializableInstruction;

pub const INSTRUCTION_OPTION_TRADE_DATA_SIZE: usize = 632;
pub const INSTRUCTION_OPTION_TRADE_MAX_BASKET_LENGTH: usize = 30;
pub const INSTRUCTION_OPTION_TRADE_ACCOUNTS_NUMBER: usize = 16;
pub const INSTRUCTION_OPTION_TRADE_ACCOUNTS_SIZE: usize =
    INSTRUCTION_ACCOUNT_INFO_REF_SIZE * INSTRUCTION_OPTION_TRADE_ACCOUNTS_NUMBER;
pub const INSTRUCTION_OPTION_TRADE_VERSION: u8 = 2;
pub const DEFAULT_OPTION_TRADE_MAX_COST: i64 = -1_000_000_000;

#[repr(C)]
// Structure alignment - 64 bit.
pub struct InstructionOptionTrade {
    pub cmd: u8,            // 1 byte (1/8 bytes align)
    pub version: u8,        // 1 byte (2/8 bytes align)
    pub reserved: u8,       // 1 byte (3/8 bytes align)
    pub basket_length: u8,  // 1 byte (4/8 bytes align)
    pub trade_qty: [i32; BUCKETS_COUNT], // 4*95=380 bytes (8/8 bytes align)
    pub max_cost: i64,      // 8 bytes
    pub basket: [OptionTradeBasketData; INSTRUCTION_OPTION_TRADE_MAX_BASKET_LENGTH], // 8*30=240 bytes
}


impl<'a> DvlDeserializableInstruction<'a> for InstructionOptionTrade {
    #[inline(always)]
    fn expected_size() -> usize {INSTRUCTION_OPTION_TRADE_DATA_SIZE}
    #[inline(always)]
    fn expected_version() -> u8 {INSTRUCTION_OPTION_TRADE_VERSION}
}

#[cfg(not(feature = "on-chain"))]
#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use crate::instructions_data::common::INSTRUCTION_SIZE_LIMIT;
    use crate::instructions_data::constructors::option_trade::OptionTradeParams;
    use crate::instructions_data::dvl_instruction_data::DvlInstruction;
    use crate::utils::option_trade_basket_data::OPTION_TRADE_BASKET_DATA_SIZE;
    use crate::utils::type_size_helper::align_size;

    pub const INSTR_OPTION_TRADE_CMD_OFFSET: usize = 0;
    pub const INSTR_OPTION_TRADE_VERSION_OFFSET: usize = 1;
    pub const INSTR_OPTION_TRADE_RESERVED_OFFSET: usize = 2;
    pub const INSTR_OPTION_TRADE_BASKET_LENGTH_OFFSET: usize = 3;
    pub const INSTR_OPTION_TRADE_TRADE_QTY_OFFSET: usize = 4;
    pub const INSTR_OPTION_TRADE_MAX_COST_OFFSET: usize = 384;
    pub const INSTR_OPTION_TRADE_BASKET_DATA_OFFSET: usize = 392;

    #[test]
    fn test_instruction_data_offsets() {
        assert_eq!(mem::size_of::<OptionTradeBasketData>(), OPTION_TRADE_BASKET_DATA_SIZE);

        let trade_params = OptionTradeParams {
            trade_qty: [0; BUCKETS_COUNT],
            basket: None,
            max_cost: None,
        };

        let data = DvlInstruction::new::<InstructionOptionTrade>(trade_params).unwrap();

        let base_ptr = &*data as *const _ as usize;

        assert_eq!(&data.cmd as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_CMD_OFFSET);
        assert_eq!(&data.version as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_VERSION_OFFSET);
        assert_eq!(&data.reserved as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_RESERVED_OFFSET);
        assert_eq!(&data.basket_length as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_BASKET_LENGTH_OFFSET);
        assert_eq!(&data.trade_qty as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_TRADE_QTY_OFFSET);
        assert_eq!(&data.max_cost as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_MAX_COST_OFFSET);
        assert_eq!(&data.basket as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_BASKET_DATA_OFFSET);

        let structure_size = mem::size_of::<InstructionOptionTrade>();
        assert_eq!(structure_size, INSTRUCTION_OPTION_TRADE_DATA_SIZE);
        assert_eq!(structure_size, align_size(structure_size, 8));
        assert_eq!(structure_size + INSTRUCTION_OPTION_TRADE_ACCOUNTS_SIZE <= INSTRUCTION_SIZE_LIMIT, true);
    }
}