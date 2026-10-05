use vstd::prelude::*;
verus! {

pub type Int64 = i64;

pub enum DrtmGetErrorReturnCode {
    NotSupported,
    NotFound,
}

pub struct S {
    pub drtm_supported: bool,
    pub recorded_launch_error_code_found: bool,
    pub recorded_launch_error_code: Int64,
}

pub spec const SUCCESS: Result<(), DrtmGetErrorReturnCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), DrtmGetErrorReturnCode> = Err(DrtmGetErrorReturnCode::NotSupported);
pub spec const NOT_FOUND: Result<(), DrtmGetErrorReturnCode> = Err(DrtmGetErrorReturnCode::NotFound);

pub open spec fn IsDrtmSupported(s: S) -> bool;

pub open spec fn IsRecordedLaunchErrorCodeFound(s: S) -> bool;

pub open spec fn ResultEqual(a: Result<(), DrtmGetErrorReturnCode>, b: Result<(), DrtmGetErrorReturnCode>) -> bool;

pub open spec fn RecordedLaunchErrorCode(s: S) -> Int64;

pub open spec fn RecordedLaunchErrorCodeUnchanged(s: S) -> bool;

} // verus!
