use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type SensorKind = u32;

pub struct S {
    pub sensor_fault_mask: u32,
    pub telemetry_collected: bool,
    pub des_via_shmti_or_fast_channel: bool,
}

pub const DE: SensorKind = 1;

pub const HARDWARE_ERROR: Int32 = -5;
pub const PARTIAL_ERROR: Int32 = -9;

pub open spec fn SensorHasFault(s: S, sensor: SensorKind) -> bool;

pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

pub open spec fn AllTelemetryDataCollected(s: S) -> bool;

pub open spec fn AllEnabledDesCollectedViaShmtiOrFastChannel(s: S) -> bool;

pub open spec fn PayloadContainsLatestDeValues(array: [UInt32; 1]) -> bool;

} // verus!
