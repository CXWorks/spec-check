use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt16 = u16;

pub struct S {
    pub dummy: u8,
}

pub open spec fn NumPinGroups() -> UInt16;
pub open spec fn NumPins() -> UInt16;
pub open spec fn NumFunctions() -> UInt16;

} // verus!
