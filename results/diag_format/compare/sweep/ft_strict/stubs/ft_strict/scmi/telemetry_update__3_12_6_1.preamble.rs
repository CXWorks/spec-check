use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub placeholder: int,
}

pub enum RmiStatusCode {
    Success,
    ErrorInput,
    ErrorHardware,
    ErrorPartial,
}

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool;
}

pub const HARDWARE_ERROR: Int32 = -1;
pub const PARTIAL_ERROR: Int32 = -2;

pub open spec fn DeHardwareFaultDetected(s: S) -> bool;
pub open spec fn SomeTelemetryDataNotCollected(s: S) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn AllEnabledDesOnShmtiOrFastChannels(s: S) -> bool;
pub open spec fn ArrayLength(array: [UInt32; 1]) -> UInt32;
pub open spec fn ArrayHoldsEnabledDesNotOnShmtiOrFastChannels(array: [UInt32; 1]) -> bool;
pub open spec fn ProvidesLatestDeValues(array: [UInt32; 1]) -> bool;
pub open spec fn AverageNotificationPeriod(s: S) -> int;
pub open spec fn ConfiguredSamplingPeriod(s: S) -> int;

} // verus!
