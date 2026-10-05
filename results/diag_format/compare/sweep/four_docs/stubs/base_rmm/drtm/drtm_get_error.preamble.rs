use vstd::prelude::*;
verus! {

pub type Int64 = i64;

pub struct S {
    pub recorded_launch_error_code: Int64,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const NOT_FOUND: Int64 = -2;

pub open spec fn IsDrtmSupported() -> bool;

pub open spec fn IsRecordedLaunchErrorCodeFound() -> bool;

pub open spec fn RecordedLaunchErrorCode() -> Int64;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

} // verus!
