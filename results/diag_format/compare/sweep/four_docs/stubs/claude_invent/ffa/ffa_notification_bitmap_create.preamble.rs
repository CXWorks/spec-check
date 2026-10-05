use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type FfaErrorCode = i32;

pub const NOT_SUPPORTED: FfaErrorCode = -1;
pub const INVALID_PARAMETERS: FfaErrorCode = -2;
pub const DENIED: FfaErrorCode = -6;
pub const NO_MEMORY: FfaErrorCode = -3;

pub struct S {
    pub dummy: int,
}

pub open spec fn FfaFunctionImplementedAtInstance(s: S, function_id: UInt32) -> bool;

pub open spec fn FfaVmIdIsRecognized(s: S, vm_id: UInt32) -> bool;

pub open spec fn FfaNotificationBitmapExists(s: S, vm_id: UInt32) -> bool;

pub open spec fn FfaNotificationBitmapMemoryAvailable(s: S, vcpu_count: UInt32, notification_count: UInt32) -> bool;

pub open spec fn FfaNotificationBitmapSpNotificationCount(s: S, vm_id: UInt32) -> UInt32;

} // verus!
