use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn ClockExists(s: S, clock_id: UInt32) -> bool;
pub open spec fn ClockExtendedNameSupported(s: S, clock_id: UInt32) -> bool;
pub open spec fn IsNullTerminatedAscii(name: Seq<u8>) -> bool;
pub open spec fn ClockExtendedName(s: S, clock_id: UInt32) -> Seq<u8>;

} // verus!
