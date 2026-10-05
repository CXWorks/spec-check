use vstd::prelude::*;

verus! {

pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub type EndpointId = UInt16;
pub type EndpointStatus = UInt32;

pub struct S {
    pub sender_id: EndpointId,
    pub receiver_id: EndpointId,
    pub msg_type: UInt32,
    pub reserved: UInt32,
    pub msg_subtype: UInt32,
}

pub const INVALID_PARAMETERS: UInt32 = 1;
pub const DENIED: UInt32 = 2;
pub const NOT_SUPPORTED: UInt32 = 3;
pub const BUSY: UInt32 = 4;
pub const ABORTED: UInt32 = 5;
pub const NOT_READY: UInt32 = 6;

pub const FFA_MSG_SEND_DIRECT_REQ: UInt32 = 0x8400006F;
pub const FFA_MSG_SEND_DIRECT_RESP: UInt32 = 0x84000070;
pub const FFA_INTERRUPT: UInt32 = 0x84000062;
pub const FFA_YIELD: UInt32 = 0x8400006C;
pub const FFA_SUCCESS: UInt32 = 0x84000061;
pub const FFA_ERROR: UInt32 = 0x84000060;

pub const RUNNING: EndpointStatus = 100;
pub const BLOCKED: EndpointStatus = 101;
pub const PREEMPTED: EndpointStatus = 102;
pub const WAITING: EndpointStatus = 103;

pub open spec fn IsValidEndpointId(s: S, id: EndpointId) -> bool;
pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;
pub open spec fn AreValidMessageFlags(s: S, msg_type: UInt32, reserved: UInt32, msg_subtype: UInt32) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S) -> bool;
pub open spec fn EndpointSupportsDirectReqReceipt(s: S, id: EndpointId) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;
pub open spec fn EndpointState(s: S, id: EndpointId) -> EndpointStatus;
pub open spec fn EndpointHasAborted(s: S, id: EndpointId) -> bool;
pub open spec fn EndpointIsReady(s: S, id: EndpointId) -> bool;

} // verus!
