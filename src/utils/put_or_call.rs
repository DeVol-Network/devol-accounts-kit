use serde::{Deserialize, Serialize};

#[repr(u32)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
pub enum PutOrCall {
    PUT = 0,
    CALL = 1,
}
