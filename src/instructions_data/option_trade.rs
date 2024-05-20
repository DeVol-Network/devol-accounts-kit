use serde::{Deserialize, Serialize};
use crate::constants::BUCKETS_COUNT;
use crate::instructions_data::dvl_deserializable_instruction::DvlDeserializableInstruction;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct BasketData {
    pub strike: u32,
    pub pc: u32,
    pub amount: i32,
}

pub const INSTRUCTION_OPTION_TRADE_DATA_SIZE: usize = 440;
pub const INSTRUCTION_OPTION_TRADE_VERSION: u8 = 2;
pub const DEFAULT_OPTION_TRADE_MAX_COST: i64 = -1_000_000_000;

#[repr(C)]
pub struct InstructionOptionTrade {
    pub cmd: u8,
    pub version: u8,
    pub reserved: u8,
    pub basket_length: u8,
    pub trade_qty: [i32; BUCKETS_COUNT],
    pub max_cost: i64,
    pub basket: [BasketData; INSTR_OPTION_TRADE_MAX_BASKET_LENGTH],
}

pub const INSTR_OPTION_TRADE_MAX_BASKET_LENGTH: usize = 4;

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
    use crate::instructions_data::constructors::option_trade::OptionTradeParams;
    use crate::instructions_data::dvl_instruction_data::DvlInstruction;

    pub const INSTR_OPTION_TRADE_CMD_OFFSET: usize = 0;
    pub const INSTR_OPTION_TRADE_VERSION_OFFSET: usize = 1;
    pub const INSTR_OPTION_TRADE_RESERVED_OFFSET: usize = 2;
    pub const INSTR_OPTION_TRADE_BASKET_LENGTH_OFFSET: usize = 3;
    pub const INSTR_OPTION_TRADE_TRADE_QTY_OFFSET: usize = 4;
    pub const INSTR_OPTION_TRADE_MAX_COST_OFFSET: usize = 384;
    pub const INSTR_OPTION_TRADE_BASKET_DATA_OFFSET: usize = 392;

    #[test]
    fn test_instruction_data_offsets() {
        assert_eq!(mem::size_of::<BasketData>(), 12);

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

        assert_eq!(mem::size_of::<InstructionOptionTrade>(), INSTRUCTION_OPTION_TRADE_DATA_SIZE);
    }
}