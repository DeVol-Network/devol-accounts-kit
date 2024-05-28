use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct BasketData {
    pub strike: u32,
    pub pc: u32,
    pub amount: i32,
}