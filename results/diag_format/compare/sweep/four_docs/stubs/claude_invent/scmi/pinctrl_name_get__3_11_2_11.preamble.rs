use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub pins: Seq<u32>,
    pub groups: Seq<u32>,
    pub functions: Seq<u32>,
}

pub const SUCCESS: Int32 = 0i32;
pub const NOT_FOUND: Int32 = -2i32;

pub open spec fn PinctrlIdentifierExists(s: S, identifier: UInt32, kind: UInt32) -> bool;

} // verus!
