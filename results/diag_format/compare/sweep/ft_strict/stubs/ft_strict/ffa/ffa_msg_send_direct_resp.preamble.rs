use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct Endpoint {
    pub state: u32,
    pub direct_msg: Seq<u32>,
}

pub struct S {
    pub endpoints: Map<UInt32, Endpoint>,
}

pub const FFA_SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -6;
pub const ABORTED: Int32 = -8;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn IsValidEndpointId(s: S, id: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn IsValidMessageFlags(s: S, flags: UInt32) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S, id: UInt32) -> bool;
pub open spec fn CallerAllowedToInvoke(s: S, id: UInt32, func_id: u32) -> bool;
pub open spec fn SupportsDirectResponseReceipt(s: S, id: UInt32) -> bool;
pub open spec fn IsImplementedAtFfaInstance(s: S, func_id: u32) -> bool;
pub open spec fn ReceiverEncounteredUnexpectedError(s: S, id: UInt32) -> bool;
pub open spec fn CompletesAsFfaMsgWait(s: S, result: Int32) -> bool;
pub open spec fn DirectResponseDelivered(s: S, receiver: UInt32, sender: UInt32, flags: UInt32) -> bool;
pub open spec fn EndpointRuns(s: S, id: UInt32) -> bool;
pub open spec fn EndpointWaitsForNewMessage(s: S, id: UInt32) -> bool;
pub open spec fn EndpointAt(s: S, id: UInt32) -> Endpoint;

} // verus!
