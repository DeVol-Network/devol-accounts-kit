use pyth_sdk_solana::state::SolanaPriceAccount;
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::sysvar::Sysvar;
use crate::accounts::oracles::oracle_params::{DataLen, Endian, OracleParams};
use crate::accounts::oracles::oracle_provider::OracleProvider;
use crate::dvl_error::DvlError;
use crate::errors::*;

pub const ORACLE_PARAMS_QUANTITY: usize = 3;
pub const ORACLES_DATA_SIZE: usize = 216;
pub const ORACLES_DATA_COUNT: usize = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OracleData {
    pub base_ticker: [u8; 8],              // The base currency ticker, e.g., BTC in BTC/USD
    pub relative_ticker: [u8; 8],          // The quote currency ticker, e.g., USD in BTC/USD
    pub configured: bool,                  // Indicates if the oracle is configured and operational
    pub use_relative_oracle: bool,         // Flag to indicate whether to use the relative_oracle_num
    pub relative_oracle_num: u8,           // Index of the reference oracle for price recalculations
    pub reserved: u8,                      // Reserved space for future use
    pub max_price_deviation: u32,          // Maximum absolute deviation for the price
    pub params: [OracleParams; ORACLE_PARAMS_QUANTITY], // Oracle parameters, 192 bytes, offset=24
}

impl OracleData {
    /// Calculates the asset price based on oracle data
    /// Returns Result<(f64, DvlError), ProgramError> where f64 is the average price
    pub fn get_asset_price(&self, ext_oracles_accounts: &[AccountInfo]) -> Result<f64, DvlError> {
        let current_time = Clock::get().
            map_err(|_| DvlError::new(ContractError::TimeReadError))?
            .unix_timestamp;
        let mut prices = Vec::<f64>::new();

        for account in ext_oracles_accounts {
            for oracle_param in self.params.iter() {
                if account.key == &oracle_param.account {
                    if !oracle_param.enabled {
                        continue;
                    }
                    match oracle_param.provider {
                        OracleProvider::Custom => {
                            let timestamp_offset = oracle_param.timestamp.offset as usize;
                            let ext_oracle_data = account.try_borrow_data().
                                map_err(|_| DvlError::new_with_account(AccountTag::ExternalOracle, ContractError::AccountSize))?;
                            let timestamp = Self::read_value(&ext_oracle_data[timestamp_offset..], oracle_param.timestamp.data_len, oracle_param.timestamp.endian)
                                .map_err(|_| DvlError::new_with_account(AccountTag::Oracle, ContractError::TimeReadError))? as i64;

                            if (current_time - timestamp).abs() > oracle_param.max_timestamp_diff_sec as i64 {
                                continue; // Skip this oracle due to time difference
                            }

                            let mantissa_offset = oracle_param.mantissa.offset as usize;
                            let mantissa = Self::read_value(&ext_oracle_data[mantissa_offset..], oracle_param.mantissa.data_len, oracle_param.mantissa.endian)?;

                            let exponent_offset = oracle_param.exponent.offset as usize;
                            let exponent = Self::read_value(&ext_oracle_data[exponent_offset..], oracle_param.exponent.data_len, oracle_param.exponent.endian)? as i32;

                            let price = mantissa as f64 * 10f64.powi(-exponent);
                            prices.push(price);
                        }
                        OracleProvider::Switchboard => {
                            continue;}
                        OracleProvider::Pyth => {
                            let price_feed = SolanaPriceAccount::account_info_to_feed( &account ).
                                map_err(|_| {
                                    DvlError::new_with_account(AccountTag::Oracle, ContractError::AssetPriceUnavailable)
                                })?;
                            let current_price = price_feed.get_price_no_older_than(current_time, oracle_param.max_timestamp_diff_sec as u64).unwrap();
                            prices.push(current_price.price as f64 * 10f64.powi(current_price.expo));
                        }
                    }
                }
            }
        }

        return match prices.len() {
            0 => Err(DvlError::new(ContractError::AssetPriceUnavailable)),
            2 => {
                if (prices[1] - prices[0]).abs() > self.max_price_deviation as f64 {
                    return Err(DvlError::new(ContractError::PriceDiscrepancyError))
                }
                Ok(prices[0])
            }
            3 => {
                let max_deviation = self.max_price_deviation as f64;
                if (prices[1] - prices[0]).abs() > max_deviation || (prices[2] - prices[1]).abs() > max_deviation {
                    return Err(DvlError::new(ContractError::PriceDiscrepancyError))
                }
                Ok(prices[0])
            }
            _ => Ok(prices[0])
        };
    }

    /// Reads value from data slice according to DataLen and Endian
    fn read_value(data: &[u8], data_len: DataLen, endian: Endian) -> Result<u128, DvlError> {
        let val = match data_len {
            DataLen::U8 => data[0] as u128,
            DataLen::U32 => {
                if endian == Endian::LE {
                    u32::from_le_bytes(data[..4].try_into().unwrap()) as u128
                } else {
                    u32::from_be_bytes(data[..4].try_into().unwrap()) as u128
                }
            },
            DataLen::U64 => {
                if endian == Endian::LE {
                    u64::from_le_bytes(data[..8].try_into().unwrap()) as u128
                } else {
                    u64::from_be_bytes(data[..8].try_into().unwrap()) as u128
                }
            },
            DataLen::U128 => {
                if endian == Endian::LE {
                    u128::from_le_bytes(data[..16].try_into().unwrap())
                } else {
                    u128::from_be_bytes(data[..16].try_into().unwrap())
                }
            },
            _ => return Err(DvlError::new(ContractError::ComputationError)), // Unsupported data length
        };
        Ok(val)
    }

}

impl Default for OracleData {
    fn default() -> Self {
        Self {
            base_ticker: [0; 8],
            relative_ticker: [0; 8],
            configured: false,
            use_relative_oracle: false,
            relative_oracle_num: 0,
            max_price_deviation: 0,
            reserved: 0,
            params: [OracleParams::default(); ORACLE_PARAMS_QUANTITY],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_instruments_account_offsets() {
        // checking total size
        assert_eq!(mem::size_of::<OracleData>(), ORACLES_DATA_SIZE);
    }
}
