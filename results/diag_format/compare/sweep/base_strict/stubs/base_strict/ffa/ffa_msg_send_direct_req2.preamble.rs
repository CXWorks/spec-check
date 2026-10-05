use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type EndpointId = u16;
pub type RtState = u8;

pub struct S {
    pub sender: EndpointId,
    pub receiver: EndpointId,
    pub uuid_lo_val: UInt64,
    pub uuid_hi_val: UInt64,
}

pub spec const NOT_SUPPORTED: Int32 = (-1int) as Int32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as Int32;
pub spec const BUSY: Int32 = (-4int) as Int32;
pub spec const DENIED: Int32 = (-6int) as Int32;
pub spec const ABORTED: Int32 = (-8int) as Int32;
pub spec const NOT_READY: Int32 = (-10int) as Int32;

pub spec const FFA_SUCCESS: UInt32 = 0x84000061;
pub spec const FFA_INTERRUPT: UInt32 = 0x84000062;
pub spec const FFA_YIELD: UInt32 = 0x8400006C;
pub spec const FFA_MSG_SEND_DIRECT_REQ2: UInt32 = 0xC400008D;
pub spec const FFA_MSG_SEND_DIRECT_RESP2: UInt32 = 0xC400008E;

pub spec const RUNNING: RtState = 1;
pub spec const BLOCKED: RtState = 2;
pub spec const PREEMPTED: RtState = 3;

pub open spec fn sender_id(s: S) -> EndpointId;
pub open spec fn receiver_id(s: S) -> EndpointId;
pub open spec fn uuid_lo(s: S) -> UInt64;
pub open spec fn uuid_hi(s: S) -> UInt64;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn IsValidEndpointId(s: S, id: EndpointId) -> bool;
pub open spec fn IsRecognizedUuid(s: S, id: EndpointId, lo: UInt64, hi: UInt64) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S, id: EndpointId) -> bool;
pub open spec fn CallerMayInvokeDirectReq2(s: S, id: EndpointId) -> bool;
pub open spec fn SupportsDirectReqReceipt(s: S, id: EndpointId) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, fid: UInt32) -> bool;
pub open spec fn RuntimeState(s: S, id: EndpointId) -> RtState;
pub open spec fn EndpointHasAborted(s: S, id: EndpointId) -> bool;
pub open spec fn EndpointReadyForRequest(s: S, id: EndpointId) -> bool;
pub open spec fn MustResumeViaFfaRun(s: S, id: EndpointId) -> bool;
pub open spec fn OtherParamRegistersAreZero(old_s: S, new_s: S) -> bool;

} // verus!
