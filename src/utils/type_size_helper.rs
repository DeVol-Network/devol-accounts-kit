pub fn align_size(size: usize, align_bytes: usize) -> usize {
    (size + align_bytes - 1) & !(align_bytes - 1)
}