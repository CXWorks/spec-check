use vstd::prelude::*;

verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -6;

pub open spec fn ClockExists(s: S, clock_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn ClockExtendedName(s: S, clock_id: UInt32) -> [UInt8; 64];

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 64], len: int) -> bool;

} // verus!
