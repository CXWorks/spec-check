use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type FfaErrorCode = i32;

pub struct FfaResult {
    pub func_id: u32,
    pub error_code: i32,
}

pub struct S {
    pub id: u64,
}

pub const NOT_SUPPORTED: FfaErrorCode = -1;
pub const INVALID_PARAMETERS: FfaErrorCode = -2;
pub const DENIED: FfaErrorCode = -6;
pub const ABORTED: FfaErrorCode = -8;

pub open spec fn FfaFunctionImplementedAtInstance(s: S, func_id: u32) -> bool;

pub open spec fn FfaResultIsError(result: FfaResult, code: FfaErrorCode) -> bool;

pub open spec fn FfaResultIsSuccess(result: FfaResult) -> bool;

pub open spec fn FfaIsValidEndpointId(s: S, id: UInt32) -> bool;

pub open spec fn FfaPerVcpuNotificationsSupported(s: S) -> bool;

pub open spec fn FfaNotificationBitmapExceedsSupported(s: S, bitmap: Seq<UInt64>) -> bool;

pub open spec fn FfaAnyNotificationBoundToOtherSenderOrPending(s: S, sender: UInt32, receiver: UInt32, bitmap: Seq<UInt64>) -> bool;

pub open spec fn FfaCallerAllowedToInvokeNotificationBind2(s: S, sender: UInt32, receiver: UInt32) -> bool;

pub open spec fn FfaSenderPartitionAborted(s: S, sender: UInt32) -> bool;

pub open spec fn FfaNotificationsBoundToSender(s: S, sender: UInt32, receiver: UInt32, bitmap: Seq<UInt64>, per_vcpu: bool) -> bool;

pub open spec fn FfaNotificationBindingsUnchangedExcept(old_s: S, new_s: S, receiver: UInt32, bitmap: Seq<UInt64>) -> bool;

} // verus!
