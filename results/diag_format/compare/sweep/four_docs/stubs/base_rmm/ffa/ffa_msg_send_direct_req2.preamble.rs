use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct Endpoint {
    pub state: Int32,
}

pub struct S {
    pub cmd_input_w0: UInt32,
    pub cmd_input_w1: UInt32,
    pub cmd_input_x2: UInt64,
    pub cmd_input_x3: UInt64,
}

pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const BUSY: Int32 = -4;
pub const DENIED: Int32 = -6;
pub const ABORTED: Int32 = -8;
pub const NOT_READY: Int32 = -10;

pub const RUNNING: Int32 = 1;
pub const BLOCKED: Int32 = 2;
pub const PREEMPTED: Int32 = 3;

pub const FFA_SUCCESS: UInt32 = 0x84000061;
pub const FFA_INTERRUPT: UInt32 = 0x84000062;
pub const FFA_YIELD: UInt32 = 0x8400006C;
pub const FFA_MSG_SEND_DIRECT_REQ2: UInt32 = 0xC400008D;
pub const FFA_MSG_SEND_DIRECT_RESP2: UInt32 = 0xC400008E;

pub open spec fn IsValidEndpointId(id: UInt16) -> bool;
pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn AreValidMessageFlags() -> bool;
pub open spec fn IsRecognizedUuid(lo: UInt64, hi: UInt64) -> bool;
pub open spec fn CalleeCanHandleRequest() -> bool;
pub open spec fn CallerMayInvoke(fid: UInt32) -> bool;
pub open spec fn EndpointSupportsDirectReq(id: UInt16) -> bool;
pub open spec fn IsImplementedAtInstance(fid: UInt32) -> bool;
pub open spec fn EndpointAt(s: S, id: UInt16) -> Endpoint;
pub open spec fn EndpointIsReady(s: S, id: UInt16) -> bool;
pub open spec fn CalleeInvoked(s: S) -> UInt32;
pub open spec fn DirectRespProvided(s: S) -> bool;
pub open spec fn DirectRequestInterrupted(s: S) -> bool;
pub open spec fn EndpointMustBeResumed(s: S, id: UInt16) -> bool;
pub open spec fn DirectRequestCompletedWithoutResp(s: S) -> bool;
pub open spec fn AllOtherParamsZero(s: S) -> bool;

} // verus!
