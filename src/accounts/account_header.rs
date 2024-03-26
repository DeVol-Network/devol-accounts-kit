use solana_program::pubkey::Pubkey;

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