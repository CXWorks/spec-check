use vstd::prelude::*;
verus! {

pub struct S {
    pub drtm_supported: bool,
    pub error_code_found: bool,
    pub previous_launch_occurred: bool,
    pub recorded_error_code: i64,
}

pub const SUCCESS: i64 = 0;
pub const NOT_SUPPORTED: i64 = -1;
pub const NOT_FOUND: i64 = -2;

pub open spec fn DrtmIsSupported(s: S) -> bool;

pub open spec fn DrtmErrorCodeFound(s: S) -> bool;

pub open spec fn DrtmPreviousLaunchOccurred(s: S) -> bool;

pub open spec fn DrtmRecordedErrorCode(s: S) -> i64;

} // verus!
