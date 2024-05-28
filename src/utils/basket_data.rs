use serde::{Deserialize, Serialize};
use crate::utils::put_or_call::PutOrCall;

// Traded basket element. 32bit alignment.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct BasketData {
    pub strike: u32,
    pub put_or_call: PutOrCall,
    pub amount: i32,
}

#[cfg(test)]
impl Default for BasketData {
    fn default() -> Self {
        Self {
            strike: 0,
            put_or_call: PutOrCall::PUT,
            amount: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::type_size_helper::align_size;
    use super::*;

    #[test]
    fn test_pools_log_offsets() {
        let real_size = std::mem::size_of::<BasketData>();
        assert_eq!(real_size, align_size(real_size, 4));
    }
}