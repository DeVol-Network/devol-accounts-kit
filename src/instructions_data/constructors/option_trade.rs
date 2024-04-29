use std::error::Error;
use crate::constants::BUCKETS_COUNT;
use crate::dvl_off_chain_error::DvlOffChainError;
use crate::instructions_data::dvl_instruction_data::DvlInstructionData;
use crate::instructions_data::instructions::Instructions;
use crate::instructions_data::option_trade::{BasketData, DEFAULT_OPTION_TRADE_MAX_COST, INSTR_OPTION_TRADE_MAX_BASKET_LENGTH, INSTRUCTION_OPTION_TRADE_VERSION, InstructionOptionTrade};

pub struct OptionTradeParams<'a> {
    pub trade_qty: [i32; BUCKETS_COUNT],
    pub max_cost: Option<i64>,
    pub basket: Option<&'a [BasketData]>,
}

impl<'a> DvlInstructionData<'a> for InstructionOptionTrade {
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
            version: INSTRUCTION_OPTION_TRADE_VERSION,
            reserved: 0,
            basket_length: basket_length as u8,
            trade_qty: params.trade_qty,
            max_cost: params.max_cost.unwrap_or(DEFAULT_OPTION_TRADE_MAX_COST),
            basket: basket_array,
        }))
    }
}

#[cfg(test)]
impl Default for InstructionOptionTrade {
    fn default() -> Self {
        InstructionOptionTrade {
            cmd: Instructions::OptionTrade as u8,
            basket_length: 0,
            version: 0,
            reserved: 0,
            trade_qty: [0; BUCKETS_COUNT],
            max_cost: 0,
            basket: [BasketData { strike: 0, pc: 0, amount: 0 }; 4],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions_data::dvl_instruction_data::DvlInstruction;
    use crate::instructions_data::option_trade::INSTRUCTION_OPTION_TRADE_DATA_SIZE;

    #[test]
    fn test_default_instruction_option_trade() {
        let trade_params = OptionTradeParams {
            trade_qty: [0; BUCKETS_COUNT],
            basket: None,
            max_cost: None,
        };
        let data = DvlInstruction::new::<InstructionOptionTrade>(trade_params).unwrap();
        assert_eq!(data.cmd, Instructions::OptionTrade as u8);
        assert_eq!(data.version, INSTRUCTION_OPTION_TRADE_VERSION);
        assert_eq!(data.max_cost, DEFAULT_OPTION_TRADE_MAX_COST);
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
        assert_eq!(data.version, INSTRUCTION_OPTION_TRADE_VERSION);

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
        assert_eq!(data.max_cost, DEFAULT_OPTION_TRADE_MAX_COST);
    }

    #[test]
    fn test_as_vec_le_instruction_option_trade() {
        let trade_params = OptionTradeParams {
            trade_qty: [0; BUCKETS_COUNT],
            basket: None,
            max_cost: None,
        };
        let data = DvlInstruction::new::<InstructionOptionTrade>(trade_params).unwrap();
        let buf = data.to_vec_le();
        assert_eq!(buf.len(), INSTRUCTION_OPTION_TRADE_DATA_SIZE);
    }
}