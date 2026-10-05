use vstd::prelude::*;

verus! {

pub type FfaErrorCode = i32;

pub const NOT_SUPPORTED: FfaErrorCode = -1;
pub const INVALID_PARAMETERS: FfaErrorCode = -2;
pub const DENIED: FfaErrorCode = -6;

pub struct S {
    pub dummy: u64,
}

pub open spec fn FfaMsgWaitImplementedAtInstance(s: S) -> bool;

pub open spec fn IsNsPhysicalOrVirtualInstance(s: S) -> bool;

pub open spec fn IsNsVirtualInstanceEretConduit(s: S) -> bool;

pub open spec fn IsRecognizedEndpointVcpuId(s: S, endpoint_id: u16, vcpu_id: u16) -> bool;

pub open spec fn CalleeInStateToHandleMsgWait(s: S) -> bool;

pub open spec fn IsValidFfaInstanceConduit(s: S) -> bool;

pub open spec fn CallerExecutionContextIsWaiting(s: S) -> bool;

pub open spec fn IsVirtualOrSecurePhysicalInstance(s: S) -> bool;

pub open spec fn CallerOwnsRxBuffer(s: S) -> bool;

pub open spec fn SchedulerInformedOfWaiting(s: S, endpoint_id: u16, vcpu_id: u16) -> bool;

pub open spec fn VcpuRunTimeout(s: S, endpoint_id: u16, vcpu_id: u16) -> u64;

} // verus!
