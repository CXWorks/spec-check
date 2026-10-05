use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;
pub type RtState = u8;

pub struct S {
    pub dummy: int,
}

pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -6;
pub const NOT_SUPPORTED: Int32 = -1;
pub const BUSY: Int32 = -4;
pub const ABORTED: Int32 = -8;
pub const NOT_READY: Int32 = -9;
pub const FFA_MSG_SEND_DIRECT_RESP2: Int32 = 100;
pub const FFA_INTERRUPT: Int32 = 101;
pub const FFA_YIELD: Int32 = 102;
pub const FFA_SUCCESS: Int32 = 103;

pub const FFA_MSG_SEND_DIRECT_REQ2: UInt32 = 0xC400008D;

pub const RUNNING: RtState = 1;
pub const BLOCKED: RtState = 2;
pub const PREEMPTED: RtState = 3;

pub open spec fn IsValidEndpointId(s: S, id: UInt16) -> bool;
pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn IsRecognizedUuid(s: S, receiver_id: UInt16, uuid_lo: UInt64, uuid_hi: UInt64) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S, receiver_id: UInt16) -> bool;
pub open spec fn CallerMayInvokeDirectReq2(s: S, sender_id: UInt16) -> bool;
pub open spec fn SupportsDirectReqReceipt(s: S, receiver_id: UInt16) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, fid: UInt32) -> bool;
pub open spec fn RuntimeState(s: S, id: UInt16) -> RtState;
pub open spec fn EndpointHasAborted(s: S, id: UInt16) -> bool;
pub open spec fn EndpointReadyForRequest(s: S, id: UInt16) -> bool;
pub open spec fn MustResumeViaFfaRun(s: S, id: UInt16) -> bool;
pub open spec fn OtherParamRegistersAreZero(s: S) -> bool;

} // verus!
