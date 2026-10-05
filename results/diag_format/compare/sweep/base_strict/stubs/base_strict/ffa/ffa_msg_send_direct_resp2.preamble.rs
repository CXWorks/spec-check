use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt64 = u64;

pub struct S {
    pub cmd_input_ids: UInt64,
    pub cmd_input_msg: Seq<UInt64>,
}

pub struct Endpoint {
    pub state: UInt64,
}

pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -6;
pub const NOT_SUPPORTED: Int32 = -1;
pub const ABORTED: Int32 = -8;

pub const FFA_MSG_SEND_DIRECT_RESP2: UInt64 = 0xC400008F;

pub open spec fn Bits64(x: UInt64, hi: int, lo: int) -> UInt64;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidEndpointId(id: UInt64) -> bool;

pub open spec fn CalleeCanHandleRequest(id: UInt64) -> bool;

pub open spec fn SupportsSendingDirectResp(id: UInt64) -> bool;

pub open spec fn SupportsReceivingDirectResp(id: UInt64) -> bool;

pub open spec fn IsImplementedAtInstance(func_id: UInt64) -> bool;

pub open spec fn ReceiverAbortedOnUnexpectedError(id: UInt64) -> bool;

pub open spec fn DirectRespDelivered(sender: UInt64, receiver: UInt64, msg: Seq<UInt64>) -> bool;

pub open spec fn EndpointRun(id: UInt64) -> bool;

pub open spec fn WaitsForNewMessage(id: UInt64) -> bool;

pub open spec fn CompletesAsFfaMsgWait(id: UInt64) -> bool;

pub open spec fn EndpointAt(id: UInt64) -> Endpoint;

} // verus!
