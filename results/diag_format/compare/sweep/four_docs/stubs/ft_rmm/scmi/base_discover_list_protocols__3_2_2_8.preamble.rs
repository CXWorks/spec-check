use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type int32 = i32;

pub struct S {
    pub num_protocols: uint32,
    pub protocols: Seq<uint32>,
}

pub const SUCCESS: int32 = 0;
pub const INVALID_PARAMETERS: int32 = -2;

pub open spec fn IsValidSkip(s: S, skip: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn ProtocolsPackedFourPerElement(protocols: [uint32; 1], num_protocols: uint32) -> bool;

pub open spec fn ProtocolsInAscendingOrder(protocols: [uint32; 1], num_protocols: uint32) -> bool;

} // verus!
