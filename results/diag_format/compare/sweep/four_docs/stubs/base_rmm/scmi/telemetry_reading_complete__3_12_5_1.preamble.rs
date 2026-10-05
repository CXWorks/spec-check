use vstd::prelude::*;

verus! {

pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const HARDWARE_ERROR: int = 1;
pub spec const PARTIAL_ERROR: int = 2;

pub uninterp spec fn DeHardwareFaultDetected(s: S) -> bool;

pub uninterp spec fn AllTelemetryDataCollected(s: S) -> bool;

pub uninterp spec fn AllEnabledDesOnShmtiOrFastChannel(s: S) -> bool;

pub uninterp spec fn array_contains_telemetry_payload_for_enabled_des_not_on_shmti_or_fast_channel(a: &[uint32], s: S) -> bool;

pub uninterp spec fn array_contains_prologue_or_epilogue(a: &[uint32]) -> bool;

pub trait TelemetryArray {
    spec fn contains_telemetry_payload_for_enabled_des_not_on_shmti_or_fast_channel(&self, s: S) -> bool;

    spec fn contains_prologue_or_epilogue(&self) -> bool;
}

impl TelemetryArray for [uint32] {
    open spec fn contains_telemetry_payload_for_enabled_des_not_on_shmti_or_fast_channel(&self, s: S) -> bool {
        array_contains_telemetry_payload_for_enabled_des_not_on_shmti_or_fast_channel(self, s)
    }

    open spec fn contains_prologue_or_epilogue(&self) -> bool {
        array_contains_prologue_or_epilogue(self)
    }
}

} // verus!
