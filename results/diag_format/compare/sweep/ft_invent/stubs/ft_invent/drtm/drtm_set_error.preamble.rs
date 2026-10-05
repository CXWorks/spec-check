use vstd::prelude::*;
verus! {

pub type Int64 = i64;

pub enum DRTMStatusCode {
    Denied,
    InvalidParameters,
    NotSupported,
    AlreadyLaunched,
}

pub struct S {
    pub drtm_error_code: Int64,
}

pub spec const DRTM_SUCCESS: Result<(), DRTMStatusCode> = Ok(());
pub spec const DRTM_DENIED: Result<(), DRTMStatusCode> = Err(DRTMStatusCode::Denied);

pub open spec fn DRTM_error_code(s: S) -> Int64;

} // verus!
