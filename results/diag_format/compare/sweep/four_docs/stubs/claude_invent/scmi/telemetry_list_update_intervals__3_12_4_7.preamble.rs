use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub telemetry_state: nat,
}

pub const SUCCESS: i32 = 0;
pub const OUT_OF_RANGE: i32 = 1;

pub open spec fn TelemetryUpdateIntervalIndexInRange(s: S, index: UInt32, group_identifier: UInt32, flags: UInt32) -> bool;

pub open spec fn UpdateIntervalLe(a: UInt32, b: UInt32) -> bool;

} // verus!
