use crate::instructions_data::common::seeds_data::SeedsData;
use crate::instructions_data::dvl_deserializable_instruction::DvlDeserializableInstruction;

pub const INSTRUCTION_MIGRATE_ACCOUNT_DATA_SIZE: usize = 11;
pub const INSTRUCTION_MIGRATE_ACCOUNT_VERSION: u8 = 1;

#[repr(C)]
pub struct InstructionMigrateAccountData {
    cmd: u8,
    pub version: u8,
    pub buffer_seed: SeedsData,
}

impl<'a> DvlDeserializableInstruction<'a> for InstructionMigrateAccountData {
    #[inline(always)]
    fn expected_size() -> usize {INSTRUCTION_MIGRATE_ACCOUNT_DATA_SIZE}
    #[inline(always)]
    fn expected_version() -> u8 {INSTRUCTION_MIGRATE_ACCOUNT_VERSION}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use crate::instructions_data::common::seeds_data::SeedsData;

    #[test]
    fn test_migrate_account_instruction_data_size() {
        assert_eq!(
            mem::size_of::<InstructionMigrateAccountData>(),
            INSTRUCTION_MIGRATE_ACCOUNT_DATA_SIZE
        );

        let instruction_data = InstructionMigrateAccountData {
            cmd: 0,
            version: 1,
            buffer_seed: SeedsData::new([0; 8], 0),
        };

        assert_eq!(
            mem::size_of_val(&instruction_data),
            INSTRUCTION_MIGRATE_ACCOUNT_DATA_SIZE
        );
    }
}