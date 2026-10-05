use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type FfaErrorCode = i32;

pub const NOT_SUPPORTED: FfaErrorCode = -1;
pub const INVALID_PARAMETERS: FfaErrorCode = -2;
pub const DENIED: FfaErrorCode = -6;

#[allow(non_camel_case_types)]
pub enum FfaResult {
    FFA_SUCCESS,
    FFA_ERROR(FfaErrorCode),
}

pub use FfaResult::*;

pub struct S {
    pub dummy: u64,
}

} // verus!
