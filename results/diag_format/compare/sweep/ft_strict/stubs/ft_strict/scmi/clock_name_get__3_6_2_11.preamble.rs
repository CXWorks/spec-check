use vstd::prelude::*;

verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub clock_count: nat,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;

pub open spec fn ClockExists(s: S, clock_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn IsClockExtendedName(name: [UInt8; 64], clock_id: UInt32) -> bool;

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 64], len: int) -> bool;

} // verus!
