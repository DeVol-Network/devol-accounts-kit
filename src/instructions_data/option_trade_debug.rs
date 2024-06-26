use crate::constants::BUCKETS_COUNT;
use crate::instructions_data::dvl_deserializable_instruction::DvlDeserializableInstruction;
use crate::instructions_data::option_trade::{INSTRUCTION_OPTION_TRADE_DATA_SIZE, INSTRUCTION_OPTION_TRADE_VERSION, InstructionOptionTrade};
use crate::utils::option_trade_basket_data::OptionTradeBasketData;

pub const INSTRUCTION_OPTION_TRADE_DEBUG_DATA_SIZE: usize = INSTRUCTION_OPTION_TRADE_DATA_SIZE + 16;

#[repr(C)]
pub struct InstructionOptionTradeDebug {
    pub option_trade: InstructionOptionTrade,
    pub time_to_expiration: i64,
    pub underlying_price: i64,
}

pub struct OptionTradeDebugParams<'a> {
    pub trade_qty: [i32; BUCKETS_COUNT],
    pub max_cost: Option<i64>,
    pub basket: Option<&'a [OptionTradeBasketData]>,
    pub time_to_expiration: i64,
    pub underlying_price: i64,
}

impl<'a> DvlDeserializableInstruction<'a> for InstructionOptionTradeDebug {
    #[inline(always)]
    fn expected_size() -> usize {INSTRUCTION_OPTION_TRADE_DEBUG_DATA_SIZE}
    #[inline(always)]
    fn expected_version() -> u8 {INSTRUCTION_OPTION_TRADE_VERSION} // always the same as OPTION TRADE version
}

#[cfg(not(feature = "on-chain"))]
#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use crate::instructions_data::common::instruction_size_params::INSTRUCTION_SIZE_LIMIT;
    use crate::instructions_data::option_trade::{INSTRUCTION_OPTION_TRADE_ACCOUNTS_SIZE};

    pub const INSTR_OPTION_TRADE_DEBUG_OPTION_TRADE_OFFSET: usize = 0;
    pub const INSTR_OPTION_TRADE_DEBUG_TIME_TO_EXPIRATION_OFFSET: usize = INSTRUCTION_OPTION_TRADE_DATA_SIZE;
    pub const INSTR_OPTION_TRADE_DEBUG_UNDERLYING_PRICE_OFFSET: usize = INSTRUCTION_OPTION_TRADE_DATA_SIZE + 8;

    #[test]
    fn test_instruction_data_offsets() {
        let data = InstructionOptionTradeDebug::default();
        let base_ptr = &data as *const _ as usize;

        assert_eq!(&data.option_trade as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_DEBUG_OPTION_TRADE_OFFSET);
        assert_eq!(&data.time_to_expiration as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_DEBUG_TIME_TO_EXPIRATION_OFFSET);
        assert_eq!(&data.underlying_price as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_DEBUG_UNDERLYING_PRICE_OFFSET);

        let structure_size = mem::size_of::<InstructionOptionTradeDebug>();
        assert_eq!(structure_size, INSTRUCTION_OPTION_TRADE_DEBUG_DATA_SIZE);
        assert_eq!(structure_size + INSTRUCTION_OPTION_TRADE_ACCOUNTS_SIZE <= INSTRUCTION_SIZE_LIMIT, true);
    }
}
