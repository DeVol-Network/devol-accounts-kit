use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum OracleProvider {
    Custom = 0,
    Switchboard = 1,
    Pyth = 2,
}