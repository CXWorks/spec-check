use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub num_pin_groups: uint32,
    pub num_pins: uint32,
    pub num_functions: uint32,
}

pub open spec fn Bits(value: uint32, high: int, low: int) -> uint32;

pub open spec fn NumPinGroups() -> uint32;

pub open spec fn NumPins() -> uint32;

pub open spec fn NumFunctions() -> uint32;

} // verus!
