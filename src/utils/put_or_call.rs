use serde::{Deserialize, Serialize};

#[repr(u32)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
pub enum PutOrCall {
    CALL = 0,
    PUT = 1,
}
