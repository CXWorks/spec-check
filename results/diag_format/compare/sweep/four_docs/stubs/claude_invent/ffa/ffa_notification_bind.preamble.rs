use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type FfaErrorCode = i32;

pub const NOT_SUPPORTED: FfaErrorCode = -1;
pub const INVALID_PARAMETERS: FfaErrorCode = -2;
pub const DENIED: FfaErrorCode = -6;
pub const ABORTED: FfaErrorCode = -8;

pub struct S {
    pub dummy: int,
}

pub open spec fn ResultEqual(result: Result<(), FfaErrorCode>, code: FfaErrorCode) -> bool;

pub open spec fn IsFfaNotificationBindImplemented(s: S) -> bool;

pub open spec fn IsValidFfaEndpointId(s: S, id: int) -> bool;

pub open spec fn IsPerVcpuNotificationSupported(s: S) -> bool;

pub open spec fn IsAnyNotificationBoundToOtherSenderOrPending(s: S, receiver: int, sender: int, bitmap: int) -> bool;

pub open spec fn IsCallerAllowedNotificationBind(s: S, id: int) -> bool;

pub open spec fn IsFfaPartitionAborted(s: S, id: int) -> bool;

pub open spec fn NotificationsBoundToSender(s: S, receiver: int, sender: int, bitmap: int, per_vcpu: bool) -> bool;

} // verus!
