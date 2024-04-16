use std::error::Error;
use crate::constants::BUCKETS_COUNT;
use crate::dvl_off_chain_error::DvlOffChainError;
use crate::instructions::devol_instruction_data::DevolInstructionData;
use crate::instructions::instructions::Instructions;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
#[repr(C)]
struct BasketData {
    strike: u32,      //  4 bytes, offset: 0
    pc: u32,          //  4 bytes, offset: 4
    amount: i32,      //  4 bytes, offset: 8
}

pub const INSTRUCTION_OPTION_TRADE_DATA_SIZE: usize = 440;

#[repr(C)]
pub struct InstructionOptionTrade {
    cmd: u8,
    version: u8,
    reserved: u8,
    basket_length: u8,
    trade_qty: [i32; BUCKETS_COUNT],
    max_cost: i64,
    basket: [BasketData; INSTR_OPTION_TRADE_MAX_BASKET_LENGTH],
}

pub struct OptionTradeParams<'a> {
    trade_qty: [i32; BUCKETS_COUNT],
    max_cost: Option<i64>,
    basket: Option<&'a[BasketData]>,
}

impl<'a> DevolInstructionData<'a> for InstructionOptionTrade {
    type DvlInstrParams = OptionTradeParams<'a>;

    fn new(params: Self::DvlInstrParams) -> Result<Box<InstructionOptionTrade>, Box<dyn Error>> {
        let basket_length = params.basket.map_or(0, |basket| basket.len());
        if basket_length > INSTR_OPTION_TRADE_MAX_BASKET_LENGTH {
            return Err(Box::new(DvlOffChainError::BasketTooLarge));
        }

        let mut basket_array = [BasketData { strike: 0, pc: 0, amount: 0 }; INSTR_OPTION_TRADE_MAX_BASKET_LENGTH];

        if let Some(basket) = params.basket {
            for (dest, src) in basket_array.iter_mut().zip(basket.iter()) {
                *dest = *src;
            }
        }

        Ok(Box::new(InstructionOptionTrade {
            cmd: Instructions::OptionTrade as u8,
            version: INSTRUCTION_VERSION,
            reserved: 0,
            basket_length: basket_length as u8,
            trade_qty: params.trade_qty,
            max_cost: params.max_cost.unwrap_or(DEFAULT_MAX_COST),
            basket: basket_array,
        }))
    }
}

const INSTRUCTION_VERSION: u8 = 2;
const DEFAULT_MAX_COST: i64 = -1_000_000_000;

#[cfg(test)]
impl Default for InstructionOptionTrade{
    fn default() -> Self {
        InstructionOptionTrade{
            cmd: 0,
            basket_length: 0,
            version: INSTRUCTION_VERSION,
            reserved: 0,
            trade_qty: [0; BUCKETS_COUNT],
            max_cost: 0,
            basket: [BasketData { strike: 0, pc: 0, amount: 0 }; 4],
        }
    }
}

pub const INSTR_OPTION_TRADE_CMD_OFFSET: usize = 0;
pub const INSTR_OPTION_TRADE_VERSION_OFFSET: usize = 1;
pub const INSTR_OPTION_TRADE_RESERVED_OFFSET: usize = 2;
pub const INSTR_OPTION_TRADE_BASKET_LENGTH_OFFSET: usize = 3;
pub const INSTR_OPTION_TRADE_TRADE_QTY_OFFSET: usize = 4;
pub const INSTR_OPTION_TRADE_MAX_COST_OFFSET: usize = 384;
pub const INSTR_OPTION_TRADE_BASKET_DATA_OFFSET: usize = 392;
pub const INSTR_OPTION_TRADE_MAX_BASKET_LENGTH: usize = 4;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use crate::instructions::devol_instruction_data::DvlInstruction;

    #[test]
    fn test_instruction_data_offsets() {
        assert_eq!(mem::size_of::<BasketData>(), 12);

        let trade_params = OptionTradeParams{
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

    #[test]
    fn test_default_instruction_option_trade() {
        let trade_params = OptionTradeParams {
            trade_qty: [0; BUCKETS_COUNT],
            basket: None,
            max_cost: None,
        };
        let data = DvlInstruction::new::<InstructionOptionTrade>(trade_params).unwrap();
        assert_eq!(data.cmd, Instructions::OptionTrade as u8);
        assert_eq!(data.version, INSTRUCTION_VERSION);
        assert_eq!(data.max_cost, DEFAULT_MAX_COST);
    }

    #[test]
    fn test_filled_basket_instruction_option_trade() {
        let custom_basket_data = [
            BasketData { strike: 10, pc: 20, amount: -30 },
            BasketData { strike: 40, pc: 50, amount: -60 },
            BasketData { strike: 70, pc: 80, amount: -90 },
        ];
        let trade_params = OptionTradeParams {
            trade_qty: [0; BUCKETS_COUNT],
            basket: Some(&custom_basket_data),
            max_cost: Some(500),
        };
        let data = DvlInstruction::new::<InstructionOptionTrade>(trade_params).unwrap();

        assert_eq!(data.cmd, Instructions::OptionTrade as u8);
        assert_eq!(data.version, INSTRUCTION_VERSION);

        assert_eq!(data.basket[0], custom_basket_data[0]);
        assert_eq!(data.basket[1], custom_basket_data[1]);
        assert_eq!(data.basket[2], custom_basket_data[2]);
        assert_eq!(data.basket[3], BasketData { strike: 0, pc: 0, amount: 0 });

        assert_eq!(data.max_cost, 500);
    }

    #[test]
    fn test_default_max_cost_instruction_option_trade() {
        let trade_params = OptionTradeParams {
            trade_qty: [0; BUCKETS_COUNT],
            basket: None,
            max_cost: None,
        };
        let data = DvlInstruction::new::<InstructionOptionTrade>(trade_params).unwrap();
        assert_eq!(data.max_cost, DEFAULT_MAX_COST);
    }
}