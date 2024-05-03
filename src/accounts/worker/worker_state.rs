use serde::{Deserialize, Serialize};

#[derive(PartialEq, PartialOrd, Copy, Clone, Serialize, Deserialize)]
#[repr(u32)]
pub enum WorkerState {
    Unassigned = 1,
    Assigned = 2,
    Inactive = 3,
    Active = 4,
}