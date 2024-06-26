use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct SvmParams {
    pub v: f64,
    pub psi: f64,
    pub p: f64,
    pub c: f64,
    pub vt: f64,
} // size: 40 bytes