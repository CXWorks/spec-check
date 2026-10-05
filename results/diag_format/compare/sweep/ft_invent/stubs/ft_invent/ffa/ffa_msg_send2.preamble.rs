use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type FfaStatusCode = i32;

pub const NOT_SUPPORTED: FfaStatusCode = -1;
pub const INVALID_PARAMETERS: FfaStatusCode = -2;
pub const NO_MEMORY: FfaStatusCode = -3;
pub const BUSY: FfaStatusCode = -4;
pub const DENIED: FfaStatusCode = -6;

pub struct S {
    pub dummy: u32,
}

pub open spec fn FFA_ERROR(code: FfaStatusCode) -> Result<(), FfaStatusCode>;

pub spec const FFA_SUCCESS: Result<(), FfaStatusCode> = Ok(());

} // verus!
