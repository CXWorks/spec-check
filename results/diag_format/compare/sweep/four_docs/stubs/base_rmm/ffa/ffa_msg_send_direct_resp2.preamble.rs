use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;

pub struct S {
    pub regs: Seq<u64>,
    pub instance_id: u32,
}

pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -6;
pub const NOT_SUPPORTED: Int32 = -1;
pub const ABORTED: Int32 = -8;

pub const FFA_MSG_SEND_DIRECT_RESP2: UInt32 = 0xC400008E;

pub open spec fn src_id(s: S) -> UInt16;

pub open spec fn dst_id(s: S) -> UInt16;

pub open spec fn IsValidEndpointId(s: S, id: UInt16) -> bool;

pub open spec fn AreValidMessageFlags(s: S) -> bool;

pub open spec fn CalleeCanHandleRequest(s: S) -> bool;

pub open spec fn CallerSupportsDirectRespSend(s: S, id: UInt16) -> bool;

pub open spec fn ReceiverSupportsDirectRespReceipt(s: S, id: UInt16) -> bool;

pub open spec fn IsImplementedAtInstance(s: S, fid: UInt32) -> bool;

pub open spec fn ReceiverAbortedOnUnexpectedError(s: S, id: UInt16) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn DirectRespMessageDeliveredTo(s: S, id: UInt16) -> bool;

pub open spec fn EndpointRun(s: S, id: UInt16) -> bool;

pub open spec fn EndpointWaitsForNewMessage(s: S, id: UInt16) -> bool;

pub open spec fn SuccessIndicatedAsFfaMsgWait(result: Int32) -> bool;

} // verus!
