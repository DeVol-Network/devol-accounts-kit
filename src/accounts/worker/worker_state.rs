use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(u32)]
pub enum WorkerState {
    Unassigned = 1,
    Assigned = 2,
    Inactive = 3,
    Active = 4,
}

impl fmt::Display for WorkerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}