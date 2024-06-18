pub const SEEDS_DATA_SIZE: usize = 9;
#[repr(C)]
pub struct SeedsData {
    seed: [u8; 8],
    pub bump_seed: [u8; 1],
}

impl SeedsData {
    pub fn new(seed: [u8; 8], bump_seed: u8) -> Self {
        SeedsData {
            seed,
            bump_seed: [bump_seed],
        }
    }

    #[inline(always)]
    pub fn get_seed(&self) -> &[u8] {
        let mut length = 0;
        while length < self.seed.len() && self.seed[length] > 0 {
            length += 1;
        }
        &self.seed[0..length]
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions_data::common::seeds_data::{SeedsData, SEEDS_DATA_SIZE};

    #[test]
    fn test_seeds_data_offsets() {
        let data = SeedsData::new([0; 8], 0);
        let base_ptr = &data as *const _ as usize;
        assert_eq!((&data.seed as *const _ as usize) - base_ptr, 0);
        assert_eq!((&data.bump_seed as *const _ as usize) - base_ptr, 8);
        assert_eq!(std::mem::size_of::<SeedsData>(), SEEDS_DATA_SIZE);
    }

    #[test]
    fn test_get_seed_all_zeros() {
        let data = SeedsData::new([0; 8], 0);
        assert_eq!(data.get_seed(), &[] as &[u8]);
    }

    #[test]
    fn test_get_seed_non_zero() {
        let data = SeedsData::new([1, 2, 3, 4, 5, 0, 0, 0], 0);
        assert_eq!(data.get_seed(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_get_seed_all_non_zeros() {
        let data = SeedsData::new([1, 2, 3, 4, 5, 6, 7, 8], 0);
        assert_eq!(data.get_seed(), &[1, 2, 3, 4, 5, 6, 7, 8]);
    }
}