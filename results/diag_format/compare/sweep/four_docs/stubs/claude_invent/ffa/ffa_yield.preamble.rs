use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type FfaErrorCode = i32;

pub type FfaFunctionId = u32;

pub struct FfaReturn {
    pub function_id: u32,
    pub error_code: i32,
}

pub struct S {
    pub state_id: int,
}

pub const NOT_SUPPORTED: FfaErrorCode = -1;
pub const INVALID_PARAMETERS: FfaErrorCode = -2;
pub const DENIED: FfaErrorCode = -6;

pub const FFA_RUN: FfaFunctionId = 0x8400006Du32;
pub const FFA_INTERRUPT: FfaFunctionId = 0x84000062u32;

pub open spec fn IsFfaYieldImplementedAtInstance(s: S) -> bool;

pub open spec fn FfaErrorEqual(result: FfaReturn, err: FfaErrorCode) -> bool;

pub open spec fn FfaFunctionEqual(result: FfaReturn, fid: FfaFunctionId) -> bool;

pub open spec fn IsEretConduit(s: S) -> bool;

pub open spec fn IsRecognizedEndpointVcpu(s: S, endpoint_id: int, vcpu_id: int) -> bool;

pub open spec fn IsCalleeInStateToHandleYield(s: S) -> bool;

pub open spec fn IsValidFfaYieldInstanceConduit(s: S) -> bool;

pub open spec fn IsEndpointVcpuIdsParamUsed(s: S) -> bool;

pub open spec fn IsHypervisorAtNonSecureVirtualEret(s: S) -> bool;

pub open spec fn CallerExecutionContextBlocked(old_s: S, new_s: S) -> bool;

pub open spec fn ExecutionYieldedToOriginalScheduler(old_s: S, new_s: S) -> bool;

pub open spec fn IsCallerSEl0Endpoint(s: S) -> bool;

pub open spec fn IsCallerVmVcpu(s: S) -> bool;

pub open spec fn VcpuRunAfterTimeout(old_s: S, new_s: S, endpoint_id: int, vcpu_id: int, timeout: int) -> bool;

} // verus!
