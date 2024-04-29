use crate::constants::BUCKETS_COUNT;
use crate::instructions_data::dvl_deserializable_instruction::DvlDeserializableInstruction;
use crate::instructions_data::option_trade::{BasketData, INSTRUCTION_OPTION_TRADE_VERSION, InstructionOptionTrade};

pub const INSTRUCTION_OPTION_TRADE_DEBUG_DATA_SIZE: usize = 456;

#[repr(C)]
pub struct InstructionOptionTradeDebug {
    pub option_trade: InstructionOptionTrade,
    pub time_to_expiration: i64,
    pub underlying_price: i64,
}

pub struct OptionTradeDebugParams<'a> {
    pub trade_qty: [i32; BUCKETS_COUNT],
    pub max_cost: Option<i64>,
    pub basket: Option<&'a [BasketData]>,
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

    pub const INSTR_OPTION_TRADE_DEBUG_OPTION_TRADE_OFFSET: usize = 0;
    pub const INSTR_OPTION_TRADE_DEBUG_TIME_TO_EXPIRATION_OFFSET: usize = 440;
    pub const INSTR_OPTION_TRADE_DEBUG_UNDERLYING_PRICE_OFFSET: usize = 448;

    #[test]
    fn test_instruction_data_offsets() {
        let data = InstructionOptionTradeDebug::default();
        let base_ptr = &data as *const _ as usize;

        assert_eq!(&data.option_trade as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_DEBUG_OPTION_TRADE_OFFSET);
        assert_eq!(&data.time_to_expiration as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_DEBUG_TIME_TO_EXPIRATION_OFFSET);
        assert_eq!(&data.underlying_price as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_DEBUG_UNDERLYING_PRICE_OFFSET);

        assert_eq!(mem::size_of::<InstructionOptionTradeDebug>(), INSTRUCTION_OPTION_TRADE_DEBUG_DATA_SIZE);
    }
}
