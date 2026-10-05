use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -4;
pub const INVALID_PARAMETERS: i32 = -2;

pub open spec fn ClockExists(s: S, clock_id: u32) -> bool;
pub open spec fn IsExtendedConfigTypeSupported(s: S, clock_id: u32, config_type: u32) -> bool;
pub open spec fn ClockIsEnabled(s: S, clock_id: u32) -> bool;
pub open spec fn ClockExtendedConfigValue(s: S, clock_id: u32, config_type: u32) -> u32;

} // verus!
