#[repr(C)]
#[derive(Copy, Clone)]
pub struct SvmParams {
    pub v: i64,
    pub psi: i64,
    pub p: i64,
    pub c: i64,
    pub vt: i64,
} // size: 40 bytes