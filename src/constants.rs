pub const BOUNDS_COUNT: usize = 94;
pub const BUCKETS_COUNT: usize = 95;

pub const VANILLA_COST_SIZE: usize = 4;
pub const VANILLA_MEMO_SIZE: usize = 49;
pub const HOURS: usize = 24;
pub const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

#[cfg(test)]
pub(crate) mod test_constants {
    cfg_if::cfg_if! {
    if #[cfg(not(feature = "on-chain"))] {
            pub const RPC_URL: &str = "https://devnet.helius-rpc.com/?api-key=a4fd5524-2f2d-4713-9acf-aeb92a7e503a";
            pub const INT_SEED: usize = 1012;
            pub const ADMIN_PUBLIC_KEY: &str = "3PvwxG6kyqKGBwYzWSvkuA8e1GqoChnmDR9WkmjJLPBg";
        }
    }
    pub const PROGRAM_ID: &str = "2aJHohZdg4oaSuXGQzSDzZC3BJvEoN5JhpBu9GERiroo";
    pub const ROOT_ADDRESS: &str = "HrWYxhCJgJ6mpBpkF1yvfdMipHBXA7iciVmGaTTz1rqE";
}
