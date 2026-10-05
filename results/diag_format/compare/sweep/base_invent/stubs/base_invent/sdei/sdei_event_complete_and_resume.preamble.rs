use vstd::prelude::*;

verus! {

pub type int64 = i64;

pub struct S {
    pub pe_id: u64,
    pub resume_addr: u64,
    pub handler_running: bool,
}

pub const SDEI_SUCCESS: int64 = 0;
pub const SDEI_ERROR_NOT_SUPPORTED: int64 = -1;
pub const SDEI_ERROR_INVALID_PARAMETERS: int64 = -2;
pub const SDEI_ERROR_DENIED: int64 = -3;

pub open spec fn SdeiSupported(s: S) -> bool;

pub open spec fn IsResumeAddressValid(s: S, addr: u64) -> bool;

pub open spec fn SdeiHandlerRunning(s: S, pe_id: u64) -> bool;

} // verus!
