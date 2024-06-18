use solana_program::pubkey::Pubkey;

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct AccountHeader {
    pub tag: u32,
    pub version: u32,
    pub root: Pubkey,
} // 40 bytes

impl Default for AccountHeader {
    fn default() -> Self {
        Self {
            version: 0,
            tag: 0,
            root: Pubkey::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pools_log_offsets() {
        let header1 = AccountHeader {
            tag: 1,
            version: 1,
            root: Pubkey::default(),
        };

        let header2 = AccountHeader {
            tag: 1,
            version: 1,
            root: Pubkey::default(),
        };

        let header3 = AccountHeader {
            tag: 2,
            version: 1,
            root: Pubkey::default(),
        };

        assert_eq!(header1, header2);
        assert_ne!(header1, header3);
    }
}