use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub const HARDWARE_ERROR: Int32 = -5;
pub const PARTIAL_ERROR: Int32 = -6;

pub open spec fn DeHardwareFaultDetected(s: S) -> bool;
pub open spec fn SomeTelemetryDataNotCollected(s: S) -> bool;
pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn AllEnabledDesOnShmtiOrFastChannels(s: S) -> bool;
pub open spec fn ArrayLength(array: [UInt32]) -> UInt32;
pub open spec fn ArrayHoldsEnabledDesNotOnShmtiOrFastChannels(array: [UInt32]) -> bool;
pub open spec fn ProvidesLatestDeValues(array: [UInt32]) -> bool;
pub open spec fn AverageNotificationPeriod(s: S) -> UInt64;
pub open spec fn ConfiguredSamplingPeriod(s: S) -> UInt64;

} // verus!
