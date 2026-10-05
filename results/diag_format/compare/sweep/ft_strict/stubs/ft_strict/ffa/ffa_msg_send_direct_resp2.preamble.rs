use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub spec const FFA_SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as i32;
pub spec const DENIED: Int32 = (-6int) as i32;
pub spec const ABORTED: Int32 = (-8int) as i32;
pub spec const result: Int32 = 1;

pub spec const FFA_MSG_SEND_DIRECT_RESP2: UInt32 = 0xC400008Fu32;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn IsValidEndpointId(s: S, id: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S, id: UInt32) -> bool;
pub open spec fn SupportsSendingDirectResp(s: S, id: UInt32) -> bool;
pub open spec fn SupportsReceivingDirectResp(s: S, id: UInt32) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;
pub open spec fn ReceiverAbortedOnUnexpectedError(s: S, id: UInt32) -> bool;
pub open spec fn DirectRespDelivered(s: S, sender: UInt32, receiver: UInt32, msg: [UInt64; 14]) -> bool;
pub open spec fn EndpointRun(s: S, id: UInt32) -> bool;
pub open spec fn WaitsForNewMessage(s: S, id: UInt32) -> bool;
pub open spec fn CompletesAsFfaMsgWait(s: S, id: UInt32) -> bool;

} // verus!
