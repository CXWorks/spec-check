use vstd::prelude::*;

verus! {

pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub type EndpointId = u16;

pub struct Message {
    pub sender: EndpointId,
    pub receiver: EndpointId,
    pub payload: Seq<UInt64>,
}

pub enum RunState {
    Running,
    Waiting,
    Blocked,
    Preempted,
    Aborted,
}

pub struct EndpointState {
    pub message: Message,
    pub run_state: RunState,
}

pub struct S {
    pub endpoints: Map<EndpointId, EndpointState>,
    pub regs: Seq<UInt64>,
}

pub const INVALID_PARAMETERS: UInt32 = 1;
pub const DENIED: UInt32 = 2;
pub const NOT_SUPPORTED: UInt32 = 3;
pub const ABORTED: UInt32 = 4;
pub const FFA_MSG_SEND_DIRECT_RESP: UInt32 = 0x84000070;

pub open spec fn source_id(s: S) -> EndpointId;
pub open spec fn dest_id(s: S) -> EndpointId;
pub open spec fn msg_type(s: S) -> UInt32;
pub open spec fn flags_rsvd(s: S) -> UInt32;
pub open spec fn fwk_msg_type(s: S) -> UInt32;
pub open spec fn impdef_args(s: S) -> Seq<UInt64>;

pub open spec fn IsValidEndpointId(s: S, id: EndpointId) -> bool;
pub open spec fn AreValidMessageFlags(s: S, msg_type: UInt32, flags_rsvd: UInt32, fwk_msg_type: UInt32) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S) -> bool;
pub open spec fn CallerMayInvokeAbi(s: S, abi: UInt32) -> bool;
pub open spec fn EndpointSupportsDirectRespReceipt(s: S, id: EndpointId) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, abi: UInt32) -> bool;
pub open spec fn EndpointAborted(s: S, id: EndpointId) -> bool;
pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;
pub open spec fn DirectRespDelivered(s: S, src: EndpointId, dst: EndpointId, msg_type: UInt32, fwk_msg_type: UInt32, args: Seq<UInt64>) -> bool;
pub open spec fn EndpointRan(s: S, id: EndpointId) -> bool;
pub open spec fn NewMessageAvailableFor(s: S, id: EndpointId) -> bool;
pub open spec fn SuccessReportedAsMsgWait(result: UInt32) -> bool;
pub open spec fn Endpoint(s: S, id: EndpointId) -> EndpointState;

} // verus!
