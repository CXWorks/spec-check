use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type FfaStatusCode = i32;

pub const FFA_SUCCESS: FfaStatusCode = 0;
pub const NOT_SUPPORTED: FfaStatusCode = -1;
pub const INVALID_PARAMETERS: FfaStatusCode = -2;
pub const DENIED: FfaStatusCode = -6;

pub struct S {
    pub dummy: int,
}

pub open spec fn FfaFunctionImplementedAtInstance(s: S, function_id: UInt32) -> bool;

pub open spec fn FfaIsRecognizedVmId(s: S, vm_id: UInt32) -> bool;

pub open spec fn FfaNotificationBitmapRegistered(s: S, vm_id: UInt32) -> bool;

pub open spec fn FfaNotificationBitmapMasked(s: S, vm_id: UInt32) -> bool;

pub open spec fn FfaNotificationBitmapPending(s: S, vm_id: UInt32) -> bool;

} // verus!
