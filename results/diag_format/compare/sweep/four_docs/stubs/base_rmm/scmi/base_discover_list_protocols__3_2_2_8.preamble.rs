use vstd::prelude::*;

verus! {

pub type int32 = i32;

pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;

pub const INVALID_PARAMETERS: int32 = -2;

pub open spec fn IsValidSkip(skip: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn ProtocolsPackedFourPerElement(protocols: Seq<uint32>, num_protocols: uint32) -> bool;

pub open spec fn ProtocolsInAscendingOrder(protocols: Seq<uint32>, num_protocols: uint32) -> bool;

} // verus!
