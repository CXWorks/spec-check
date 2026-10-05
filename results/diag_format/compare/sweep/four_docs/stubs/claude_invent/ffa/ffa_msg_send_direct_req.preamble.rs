use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type FfaErrorCode = i32;

pub const NOT_SUPPORTED: FfaErrorCode = -1i32;
pub const INVALID_PARAMETERS: FfaErrorCode = -2i32;
pub const NO_MEMORY: FfaErrorCode = -3i32;
pub const BUSY: FfaErrorCode = -4i32;
pub const INTERRUPTED: FfaErrorCode = -5i32;
pub const DENIED: FfaErrorCode = -6i32;
pub const RETRY: FfaErrorCode = -7i32;
pub const ABORTED: FfaErrorCode = -8i32;
pub const NO_DATA: FfaErrorCode = -9i32;
pub const NOT_READY: FfaErrorCode = -10i32;

pub struct FfaResult {
    pub function_id: u32,
    pub error_code: FfaErrorCode,
}

pub struct S {
    pub version: u32,
}

pub open spec fn FfaFunctionImplemented(s: S, function_id: UInt32) -> bool;
pub open spec fn FfaResultIsError(result: FfaResult, code: FfaErrorCode) -> bool;
pub open spec fn FfaResultIsAnyError(result: FfaResult) -> bool;
pub open spec fn IsValidEndpointId(s: S, id: UInt32) -> bool;
pub open spec fn IsValidFrameworkMessageType(msg_type: UInt32) -> bool;
pub open spec fn CalleeCanHandleDirectReq(s: S) -> bool;
pub open spec fn EndpointSupportsDirectReqReceipt(s: S, id: UInt32) -> bool;
pub open spec fn EndpointIsRunning(s: S, id: UInt32) -> bool;
pub open spec fn EndpointIsBlocked(s: S, id: UInt32) -> bool;
pub open spec fn EndpointIsPreempted(s: S, id: UInt32) -> bool;
pub open spec fn EndpointIsAborted(s: S, id: UInt32) -> bool;
pub open spec fn EndpointIsReadyForDirectReq(s: S, id: UInt32) -> bool;
pub open spec fn FfaResultIsDirectResp(result: FfaResult) -> bool;
pub open spec fn FfaResultIsInterrupt(result: FfaResult) -> bool;
pub open spec fn FfaResultIsYield(result: FfaResult) -> bool;
pub open spec fn FfaResultIsSuccess(result: FfaResult) -> bool;

} // verus!
