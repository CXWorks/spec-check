use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;
pub const INVALID_PARAMETERS: Int32 = -2;

pub struct ClockState {
    pub enabled: bool,
    pub config: UInt32,
    pub extended_config: UInt32,
}

pub struct S {
    pub clocks: Map<UInt32, ClockState>,
    pub supported_configs: Set<UInt32>,
}

pub open spec fn ClockExists(s: S, clock_id: UInt32) -> bool;

pub open spec fn IsSupportedConfig(s: S, flags: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Clock(s: S, clock_id: UInt32) -> ClockState;

pub open spec fn ExtendedConfigValue(s: S, clock_id: UInt32, ext_type: UInt32) -> UInt32;

} // verus!
