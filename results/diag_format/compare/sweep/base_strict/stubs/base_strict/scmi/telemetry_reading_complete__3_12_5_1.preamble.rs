use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub struct Array<T> {
    pub data: Seq<T>,
}

pub struct De {
    pub id: nat,
}

pub struct S {
    pub hardware_fault: bool,
    pub telemetry_collected: bool,
}

pub spec const SUCCESS: Int32 = 0;

pub spec const HARDWARE_ERROR: Int32 = (-1int) as i32;

pub spec const PARTIAL_ERROR: Int32 = (-2int) as i32;

pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub uninterp spec fn DeHardwareFaultDetected(s: S) -> bool;

pub uninterp spec fn AllTelemetryDataCollected(s: S) -> bool;

pub uninterp spec fn AllEnabledDesCollectedViaShmtiOrFastChannel(s: S) -> bool;

pub uninterp spec fn ArrayLength(array: Array<UInt32>) -> UInt32;

pub uninterp spec fn DeEnabled(de: De) -> bool;

pub uninterp spec fn DeAvailableViaShmtiOrFastChannel(de: De) -> bool;

pub uninterp spec fn DeDataReadableViaShmtiOrFastChannel(de: De) -> bool;

pub uninterp spec fn PayloadContainsLineForDe(array: Array<UInt32>, de: De) -> bool;

pub uninterp spec fn PayloadContainsPrologue(array: Array<UInt32>) -> bool;

pub uninterp spec fn PayloadContainsEpilogue(array: Array<UInt32>) -> bool;

pub uninterp spec fn ShmtiAndFastChannelTelemetryData(s: S) -> bool;

} // verus!
