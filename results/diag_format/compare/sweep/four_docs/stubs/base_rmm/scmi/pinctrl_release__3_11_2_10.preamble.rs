use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub identifier: UInt32,
    pub selector: UInt32,
    pub flags: UInt32,
    pub caller: UInt32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = 1;
pub spec const INVALID_PARAMETERS: Int32 = 2;

pub open spec fn identifier(s: S) -> UInt32;
pub open spec fn selector(s: S) -> UInt32;
pub open spec fn flags(s: S) -> UInt32;
pub open spec fn caller(s: S) -> UInt32;

pub open spec fn IsValidPinOrGroup(s: S, id: UInt32, sel: UInt32) -> bool;
pub open spec fn AreValidParameters(s: S, id: UInt32, fl: UInt32) -> bool;
pub open spec fn HasExclusiveControl(s: S, c: UInt32, id: UInt32, sel: UInt32) -> bool;
pub open spec fn ExclusiveControl(s: S, id: UInt32, sel: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
