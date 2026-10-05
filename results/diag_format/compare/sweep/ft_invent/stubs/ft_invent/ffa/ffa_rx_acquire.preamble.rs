use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type FfaErrorCode = i32;

pub const DENIED: FfaErrorCode = -6;
pub const INVALID_PARAMETERS: FfaErrorCode = -2;
pub const NOT_SUPPORTED: FfaErrorCode = -1;

#[allow(non_camel_case_types)]
pub enum FfaReturn {
    FFA_SUCCESS,
    FFA_ERROR(FfaErrorCode),
}

pub use FfaReturn::*;

pub struct S {
    pub dummy: int,
}

} // verus!
