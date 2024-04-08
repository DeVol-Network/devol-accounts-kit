#[repr(C)]
#[derive(Copy, Clone)]
pub struct SvmParams {
    pub v: f64,
    pub psi: f64,
    pub p: f64,
    pub c: f64,
    pub vt: f64,
} // s