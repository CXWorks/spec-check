use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = 1;

pub struct S {
    pub clock_count: nat,
}

pub open spec fn ClockExists(clock_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

pub open spec fn Bits(value: UInt32, high: int, low: int) -> int;

pub open spec fn IsClockExtendedName(name: [UInt8; 64], clock_id: UInt32) -> bool;

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 64], len: int) -> bool;

} // verus!
