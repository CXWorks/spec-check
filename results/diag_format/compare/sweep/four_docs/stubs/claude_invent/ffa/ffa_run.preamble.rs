use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type FfaErrorCode = i32;

pub const NOT_SUPPORTED: FfaErrorCode = -1;
pub const INVALID_PARAMETERS: FfaErrorCode = -2;
pub const NO_MEMORY: FfaErrorCode = -3;
pub const BUSY: FfaErrorCode = -4;
pub const INTERRUPTED: FfaErrorCode = -5;
pub const DENIED: FfaErrorCode = -6;
pub const RETRY: FfaErrorCode = -7;
pub const ABORTED: FfaErrorCode = -8;
pub const NO_DATA: FfaErrorCode = -9;
pub const NOT_READY: FfaErrorCode = -10;

pub enum FfaInstance {
    NonSecure,
    Secure,
}

pub enum FfaConduit {
    Smc,
    Hvc,
    Svc,
    Eret,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn Err(e: FfaErrorCode) -> Result<(), FfaErrorCode>;

pub open spec fn FfaRunImplemented(s: S, instance: FfaInstance) -> bool;

pub open spec fn FfaRunConduitValid(instance: FfaInstance, conduit: FfaConduit) -> bool;

pub open spec fn FfaEndpointIdRecognized(s: S, target_id: UInt32) -> bool;

pub open spec fn FfaVcpuIdRecognized(s: S, target_id: UInt32, target_vcpu: UInt32) -> bool;

pub open spec fn FfaVcpuPinnedToDifferentPe(s: S, target_id: UInt32, target_vcpu: UInt32) -> bool;

pub open spec fn FfaCalleeInStateToHandleRun(s: S, target_id: UInt32, target_vcpu: UInt32) -> bool;

pub open spec fn FfaCallerAllowedToInvokeRun(s: S, instance: FfaInstance) -> bool;

pub open spec fn FfaVcpuBusy(s: S, target_id: UInt32, target_vcpu: UInt32) -> bool;

pub open spec fn FfaVcpuAborted(s: S, target_id: UInt32, target_vcpu: UInt32) -> bool;

pub open spec fn FfaReceiverReady(s: S, target_id: UInt32, target_vcpu: UInt32) -> bool;

pub open spec fn FfaExecCtxIsWaitingBlockedOrPreempted(s: S, target_id: UInt32, target_vcpu: UInt32) -> bool;

pub open spec fn FfaExecCtxIsRunning(s: S, target_id: UInt32, target_vcpu: UInt32) -> bool;

} // verus!
