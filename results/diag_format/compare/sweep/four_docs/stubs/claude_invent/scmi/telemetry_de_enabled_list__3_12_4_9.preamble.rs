use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const OUT_OF_RANGE: i32 = 1;

pub open spec fn TelemetryEnabledElementCount(s: S, which: int) -> int;

pub open spec fn TelemetryEnabledElementId(s: S, which: int, idx: int) -> u32;

pub open spec fn TelemetryEnabledElementMode(s: S, which: int, idx: int) -> u32;

} // verus!
