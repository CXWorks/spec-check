use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const HARDWARE_ERROR: Int32 = 1;
pub const PARTIAL_ERROR: Int32 = 2;

pub open spec fn SensorHasFault(s: S) -> bool;
pub open spec fn AllTelemetryDataCollected(s: S) -> bool;
pub open spec fn AllEnabledDesCollectedViaShmtiOrFastChannel(s: S) -> bool;
pub open spec fn PayloadContainsLatestDeValues(array: [UInt32]) -> bool;

} // verus!
