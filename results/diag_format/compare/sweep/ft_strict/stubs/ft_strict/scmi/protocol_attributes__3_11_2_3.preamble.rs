use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type int32 = i32;

pub struct S {
    pub dummy: int,
}

pub open spec fn Bits(x: uint32, hi: int, lo: int) -> uint32;

pub open spec fn NumPinGroups() -> uint32;

pub open spec fn NumPins() -> uint32;

pub open spec fn NumFunctions() -> uint32;

} // verus!
