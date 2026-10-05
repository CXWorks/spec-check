use vstd::prelude::*;

verus! {

pub type UInt = u64;
pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub de_hardware_fault: bool,
    pub all_telemetry_collected: bool,
    pub all_enabled_des_on_shmti_or_fast_channel: bool,
}

pub const RSI_SUCCESS: int32 = 0;
pub const HARDWARE_ERROR: int32 = 1;
pub const PARTIAL_ERROR: int32 = 2;
pub const result: int32 = 3;

pub open spec fn DeHardwareFaultDetected(s: S) -> bool;

pub open spec fn AllTelemetryDataCollected(s: S) -> bool;

pub open spec fn AllEnabledDesOnShmtiOrFastChannel(s: S) -> bool;

pub open spec fn ResultEqual(status: int32, code: int32) -> bool;

} // verus!
