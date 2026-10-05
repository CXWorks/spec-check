use vstd::prelude::*;
verus! {

pub struct S {
    pub num_pin_groups: nat,
    pub num_pins: nat,
    pub num_functions: nat,
}

pub open spec fn PinctrlNumPinGroups(s: S) -> int;

pub open spec fn PinctrlNumPins(s: S) -> int;

pub open spec fn PinctrlNumFunctions(s: S) -> int;

} // verus!
