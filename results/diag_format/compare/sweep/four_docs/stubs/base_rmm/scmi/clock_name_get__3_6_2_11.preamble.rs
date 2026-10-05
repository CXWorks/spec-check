use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;
pub type ClockId = u32;

pub struct S {
    pub current_clock: ClockId,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = 1;

pub open spec fn clock_id(s: S) -> ClockId;

pub open spec fn ClockExists(id: ClockId) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn ClockExtendedName(id: ClockId) -> [UInt8; 64];

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 64], len: int) -> bool;

} // verus!
