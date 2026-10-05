use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const INVALID_PARAMETERS: i32 = -2;
pub const NOT_FOUND: i32 = -4;

pub open spec fn PinctrlIdentifierIsValid(s: S, identifier: UInt32, sel: int) -> bool;

pub open spec fn PinctrlExclusiveControlReleased(old_s: S, new_s: S, identifier: UInt32, sel: int) -> bool;

} // verus!
