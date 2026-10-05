use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const HARDWARE_ERROR: i32 = 1;
pub const PARTIAL_ERROR: i32 = 2;

pub open spec fn IsDeHardwareFault(s: S) -> bool;
pub open spec fn IsTelemetryPartiallyCollected(s: S) -> bool;
pub open spec fn AllEnabledDesCollectedViaShmtiOrFastChannels(s: S) -> bool;

} // verus!
