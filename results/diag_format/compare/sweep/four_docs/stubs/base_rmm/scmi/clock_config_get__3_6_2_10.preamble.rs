use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct ClockInfo {
    pub enabled: bool,
    pub rate: u64,
}

pub struct S {
    pub clock_id_field: UInt32,
    pub flags_field: UInt32,
    pub clocks: Map<UInt32, ClockInfo>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;

pub open spec fn clock_id(s: S) -> UInt32;

pub open spec fn flags(s: S) -> UInt32;

pub open spec fn ClockExists(id: UInt32) -> bool;

pub open spec fn IsSupportedConfig(f: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Clock(s: S, id: UInt32) -> ClockInfo;

pub open spec fn ExtendedConfigValue(id: UInt32, f: UInt32) -> UInt32;

} // verus!
