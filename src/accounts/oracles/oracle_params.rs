use solana_program::pubkey::Pubkey;
use crate::accounts::oracles::oracle_provider::OracleProvider;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
#[repr(u8)]
pub enum DataLen {
    Undefined,
    U8,    // 8-bit unsigned integer
    U32,   // 32-bit unsigned integer
    U64,   // 64-bit unsigned integer
    U128,  // 128-bit unsigned integer
} // size: 1 byte


#[derive(PartialEq, PartialOrd, Copy, Clone)]
#[repr(u8)]
pub enum Endian {
    LE,
    BE,
} // size: 1 byte

#[derive(Copy, Clone)]
#[repr(C)]
pub struct OracleDataField{
    pub data_len: DataLen,      // Length of the data (8, 32, 64, 128 bits)
    pub endian: Endian,         // Little or Big endian
    pub reserved: [u8; 2],      // Reserved bytes for future use or alignment
    pub offset: i32,            // Offset from the start of the data storage
} // size: 8 byte

#[cfg(test)]
impl Default for OracleDataField {
    fn default() -> Self {
        Self {
            reserved: [0; 2],
            offset: 0,
            data_len: DataLen::U32,
            endian: Endian::LE,
        }
    }
}

pub const ORACLE_DATA_FIELDS_QUANTITY: usize = 3;
pub const ORACLE_PARAMS_SIZE: usize = 64;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct OracleParams {
    pub version: u8,                    // Version of the OracleParams structure
    pub enabled: bool,                  // This Oracle has been set up
    pub provider: OracleProvider,
    pub reserved: [u8; 1],              // Reserved bytes for future use or alignment
    pub max_timestamp_diff_sec: i32,    // Maximum allowed difference in timestamp (seconds)
    pub account: Pubkey,                // Public key of the oracles account
    pub mantissa: OracleDataField,
    pub exponent: OracleDataField,
    pub timestamp: OracleDataField,
} // size: ORACLE_PARAMS_SIZE bytes

#[cfg(test)]
impl Default for OracleParams {
    fn default() -> Self {
        Self {
            account: Pubkey::default(),
            enabled: true,
            provider: OracleProvider::Pyth,
            reserved: [0; 1],
            version: 0,
            exponent: OracleDataField::default(),
            mantissa: OracleDataField::default(),
            timestamp: OracleDataField::default(),
            max_timestamp_diff_sec: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use crate::accounts::oracles::oracle_params::{ORACLE_PARAMS_SIZE, OracleParams};

    #[test]
    fn test_oracle_params() {
        assert_eq!(mem::size_of::<OracleParams>(), ORACLE_PARAMS_SIZE, "Total size of OracleParams structure mismatch");
    }
}