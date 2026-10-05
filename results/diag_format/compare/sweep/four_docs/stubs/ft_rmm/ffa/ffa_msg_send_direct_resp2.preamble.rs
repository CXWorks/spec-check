use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;

pub enum FfaStatusCode {
    InvalidParameters,
    Denied,
    NotSupported,
    Aborted,
    Busy,
    Retry,
    NoMemory,
}

pub struct S {
    pub dummy: int,
}

pub spec const INVALID_PARAMETERS: FfaStatusCode = FfaStatusCode::InvalidParameters;
pub spec const DENIED: FfaStatusCode = FfaStatusCode::Denied;
pub spec const NOT_SUPPORTED: FfaStatusCode = FfaStatusCode::NotSupported;
pub spec const ABORTED: FfaStatusCode = FfaStatusCode::Aborted;

pub spec const FFA_SUCCESS: Result<int, FfaStatusCode> = Ok(0int);

pub spec const FFA_MSG_SEND_DIRECT_RESP2: UInt32 = 0x8400008Fu32;

pub open spec fn IsValidEndpointId(s: S, id: UInt16) -> bool;
pub open spec fn ResultEqual(result: Result<int, FfaStatusCode>, code: FfaStatusCode) -> bool;
pub open spec fn AreValidMessageFlags(s: S) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S) -> bool;
pub open spec fn CallerSupportsDirectRespSend(s: S, id: UInt16) -> bool;
pub open spec fn ReceiverSupportsDirectRespReceipt(s: S, id: UInt16) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;
pub open spec fn ReceiverAbortedOnUnexpectedError(s: S, id: UInt16) -> bool;
pub open spec fn DirectRespMessageDeliveredTo(s: S, id: UInt16) -> bool;
pub open spec fn EndpointRun(s: S, id: UInt16) -> bool;
pub open spec fn EndpointWaitsForNewMessage(s: S, id: UInt16) -> bool;
pub open spec fn SuccessIndicatedAsFfaMsgWait(s: S, result: Result<int, FfaStatusCode>) -> bool;

} // verus!
