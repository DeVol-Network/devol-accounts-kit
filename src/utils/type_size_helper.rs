
pub fn round_up_to_multiple(size: usize, align_bytes: usize) -> usize {
    (size + align_bytes - 1) & !(align_bytes - 1)
}