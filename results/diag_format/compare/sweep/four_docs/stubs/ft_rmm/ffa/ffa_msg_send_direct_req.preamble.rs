use vstd::prelude::*;
verus! {

pub type UInt1 = u8;
pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt23 = u32;
pub type UInt32 = u32;
pub type UInt64 = u64;

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
    NotReady,
    Yielded,
    DirectResp,
}

pub enum EpState {
    Running,
    Blocked,
    Preempted,
    Waiting,
    Ready,
    Aborted,
}

pub struct S {
    pub dummy: nat,
}

pub spec const NOT_SUPPORTED: FfaStatusCode = FfaStatusCode::NotSupported;
pub spec const INVALID_PARAMETERS: FfaStatusCode = FfaStatusCode::InvalidParameters;
pub spec const BUSY: FfaStatusCode = FfaStatusCode::Busy;
pub spec const DENIED: FfaStatusCode = FfaStatusCode::Denied;
pub spec const ABORTED: FfaStatusCode = FfaStatusCode::Aborted;
pub spec const NOT_READY: FfaStatusCode = FfaStatusCode::NotReady;

pub spec const FFA_SUCCESS: Result<(), FfaStatusCode> = Ok(());
pub spec const FFA_INTERRUPT: Result<(), FfaStatusCode> = Err(FfaStatusCode::Interrupted);
pub spec const FFA_YIELD: Result<(), FfaStatusCode> = Err(FfaStatusCode::Yielded);
pub spec const FFA_MSG_SEND_DIRECT_RESP: Result<(), FfaStatusCode> = Err(FfaStatusCode::DirectResp);

pub spec const FFA_MSG_SEND_DIRECT_REQ: UInt32 = 0x8400006Fu32;

pub spec const RUNNING: EpState = EpState::Running;
pub spec const BLOCKED: EpState = EpState::Blocked;
pub spec const PREEMPTED: EpState = EpState::Preempted;

pub open spec fn IsValidEndpointId(s: S, id: UInt16) -> bool;
pub open spec fn AreValidMessageFlags(s: S, msg_type: UInt1, reserved: UInt23, msg_subtype: UInt8) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S) -> bool;
pub open spec fn EndpointSupportsDirectReqReceipt(s: S, id: UInt16) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;
pub open spec fn EndpointState(s: S, id: UInt16) -> EpState;
pub open spec fn EndpointHasAborted(s: S, id: UInt16) -> bool;
pub open spec fn EndpointIsReady(s: S, id: UInt16) -> bool;
pub open spec fn ResultEqual(result: Result<(), FfaStatusCode>, code: FfaStatusCode) -> bool;

} // verus!
