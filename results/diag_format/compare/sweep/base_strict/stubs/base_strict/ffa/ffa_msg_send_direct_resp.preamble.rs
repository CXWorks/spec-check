use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct Endpoint {
    pub state: UInt32,
    pub direct_msg: UInt64,
}

pub struct S {
    pub ids: UInt32,
    pub flags: UInt32,
    pub fid: UInt32,
}

pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -6;
pub const NOT_SUPPORTED: Int32 = -1;
pub const ABORTED: Int32 = -8;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidEndpointId(id: UInt32) -> bool;

pub open spec fn IsValidMessageFlags(flags: UInt32) -> bool;

pub open spec fn CalleeCanHandleRequest(id: UInt32) -> bool;

pub open spec fn CallerAllowedToInvoke(id: UInt32, fid: UInt32) -> bool;

pub open spec fn SupportsDirectResponseReceipt(id: UInt32) -> bool;

pub open spec fn IsImplementedAtFfaInstance(fid: UInt32) -> bool;

pub open spec fn ReceiverEncounteredUnexpectedError(id: UInt32) -> bool;

pub open spec fn CompletesAsFfaMsgWait(result: Int32) -> bool;

pub open spec fn DirectResponseDelivered(receiver: UInt32, sender: UInt32, flags: UInt32) -> bool;

pub open spec fn EndpointRuns(id: UInt32) -> bool;

pub open spec fn EndpointWaitsForNewMessage(id: UInt32) -> bool;

pub open spec fn EndpointAt(s: S, id: UInt32) -> Endpoint;

} // verus!
