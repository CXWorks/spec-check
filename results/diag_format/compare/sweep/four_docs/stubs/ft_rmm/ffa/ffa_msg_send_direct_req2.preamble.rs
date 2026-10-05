use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt64 = u64;
pub type Int32 = i32;

pub struct Endpoint {
    pub state: Int32,
}

pub struct S {
    pub endpoints: Seq<Endpoint>,
    pub callee_invoked: Result<(), Int32>,
}

pub const RUNNING: Int32 = 1;
pub const BLOCKED: Int32 = 2;
pub const PREEMPTED: Int32 = 3;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const BUSY: Int32 = -4;
pub const DENIED: Int32 = -6;
pub const ABORTED: Int32 = -8;
pub const NOT_READY: Int32 = -9;
pub const FFA_MSG_SEND_DIRECT_REQ2: Int32 = 141;

pub spec const FFA_SUCCESS: Result<(), Int32> = Ok(());
pub spec const FFA_MSG_SEND_DIRECT_RESP2: Result<(), Int32> = Err(142);
pub spec const FFA_INTERRUPT: Result<(), Int32> = Err(98);
pub spec const FFA_YIELD: Result<(), Int32> = Err(108);

pub open spec fn IsValidEndpointId(s: S, id: UInt16) -> bool;
pub open spec fn AreValidMessageFlags(s: S) -> bool;
pub open spec fn ResultEqual(result: Result<(), Int32>, code: Int32) -> bool;
pub open spec fn IsRecognizedUuid(s: S, uuid_lo: UInt64, uuid_hi: UInt64) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S) -> bool;
pub open spec fn CallerMayInvoke(s: S, func_id: Int32) -> bool;
pub open spec fn EndpointSupportsDirectReq(s: S, id: UInt16) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func_id: Int32) -> bool;
pub open spec fn EndpointAt(s: S, id: UInt16) -> Endpoint;
pub open spec fn EndpointIsReady(s: S, id: UInt16) -> bool;
pub open spec fn CalleeInvoked(s: S) -> Result<(), Int32>;

} // verus!
