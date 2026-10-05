use vstd::prelude::*;
verus! {

pub type Int64 = i64;

pub enum NotSupported {
    NotSupportedError,
    NotFoundError,
}

pub struct S {
    pub drtm_supported: bool,
    pub launch_error_code_found: bool,
    pub launch_error_code: Int64,
}

pub spec const SUCCESS: Result<(), NotSupported> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), NotSupported> = Err(NotSupported::NotSupportedError);
pub spec const NOT_FOUND: Result<(), NotSupported> = Err(NotSupported::NotFoundError);

pub open spec fn IsDrtmSupported(s: S) -> bool;

pub open spec fn IsRecordedLaunchErrorCodeFound(s: S) -> bool;

pub open spec fn ResultEqual(a: Result<(), NotSupported>, b: Result<(), NotSupported>) -> bool;

pub open spec fn RecordedLaunchErrorCode() -> Int64;

} // verus!
