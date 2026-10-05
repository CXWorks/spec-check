use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum FfaStatusCode {
    NotSupported,
    InvalidParameters,
    NoMemory,
    Busy,
    Interrupted,
    Denied,
    Retry,
    Aborted,
    NoData,
}

pub struct S {
    pub dummy: int,
}

pub spec const FFA_SUCCESS: Result<(), FfaStatusCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), FfaStatusCode> = Err(FfaStatusCode::InvalidParameters);
pub spec const DENIED: Result<(), FfaStatusCode> = Err(FfaStatusCode::Denied);
pub spec const NO_MEMORY: Result<(), FfaStatusCode> = Err(FfaStatusCode::NoMemory);

pub open spec fn FunctionImplementedAtInstance(s: S, func_id: u32) -> bool;
pub open spec fn ResultEqual(a: Result<(), FfaStatusCode>, b: Result<(), FfaStatusCode>) -> bool;
pub open spec fn IsRecognizedVmId(s: S, vm_id: u32, instance: u32) -> bool;
pub open spec fn NotificationBitmapExists(s: S, vm_id: u32, instance: u32) -> bool;
pub open spec fn NotificationBitmapAllocatable(s: S, vm_id: u32, instance: u32) -> bool;
pub open spec fn AllocatedSpNotificationCount(s: S, vm_id: u32, instance: u32) -> u32;

} // verus!
