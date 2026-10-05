use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type RmiStatusCode = u32;

pub const SUCCESS: RmiStatusCode = 0;
pub const NOT_FOUND: RmiStatusCode = 1;
pub const INVALID_PARAMETERS: RmiStatusCode = 2;

pub struct S {
    pub dummy: int,
}

pub open spec fn ClockExists(s: S, clock_id: UInt32) -> bool;

pub open spec fn IsValidConfigFlags(s: S, clock_id: UInt32, flags: UInt32) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn ClockEnableState(s: S, clock_id: UInt32) -> UInt32;

pub open spec fn ExtendedConfigValue(s: S, clock_id: UInt32, oem_type: int) -> UInt32;

} // verus!
