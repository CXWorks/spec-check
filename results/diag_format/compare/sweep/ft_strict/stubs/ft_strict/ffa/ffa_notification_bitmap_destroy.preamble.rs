use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

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

pub enum FfaInstance {
    NonSecure,
    Secure,
}

pub struct S {
    pub dummy: int,
}

pub spec const FFA_SUCCESS: Result<UInt32, FfaStatusCode> = Ok(0u32);
pub spec const INVALID_PARAMETERS: Result<UInt32, FfaStatusCode> = Err(FfaStatusCode::InvalidParameters);
pub spec const NOT_SUPPORTED: Result<UInt32, FfaStatusCode> = Err(FfaStatusCode::NotSupported);
pub spec const DENIED: Result<UInt32, FfaStatusCode> = Err(FfaStatusCode::Denied);

pub spec const FFA_NOTIFICATION_BITMAP_DESTROY: UInt32 = 0x8400007Eu32;

pub open spec fn IsRecognizedPartitionId(s: S, vm_id: UInt32) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32, inst: FfaInstance) -> bool;
pub open spec fn CurrentFfaInstance() -> FfaInstance;
pub open spec fn IsNotificationBitmapRegistered(s: S, vm_id: UInt32) -> bool;
pub open spec fn IsNotificationBitmapMasked(s: S, vm_id: UInt32) -> bool;
pub open spec fn IsNotificationBitmapPending(s: S, vm_id: UInt32) -> bool;
pub open spec fn ResultEqual(r1: Result<UInt32, FfaStatusCode>, r2: Result<UInt32, FfaStatusCode>) -> bool;

} // verus!
