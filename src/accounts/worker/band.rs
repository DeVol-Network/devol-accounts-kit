use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct Band {
    pub depo: i64,  // set to 0
    pub px: i64,    // will be setup on the start pool
    pub loan: i64,  // obsolete
    pub prop: i64,  // obsolete
}