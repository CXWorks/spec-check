use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub struct De {
    pub id: u64,
}

pub spec const SUCCESS: Int32 = 0;

pub spec const HARDWARE_ERROR: Int32 = 1;

pub spec const PARTIAL_ERROR: Int32 = 2;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn DeHardwareFaultDetected(s: S) -> bool;

pub open spec fn AllTelemetryDataCollected(s: S) -> bool;

pub open spec fn AllEnabledDesCollectedViaShmtiOrFastChannel(s: S) -> bool;

pub open spec fn ArrayLength(array: [UInt32; 1]) -> UInt32;

pub open spec fn DeEnabled(de: De) -> bool;

pub open spec fn DeAvailableViaShmtiOrFastChannel(de: De) -> bool;

pub open spec fn DeDataReadableViaShmtiOrFastChannel(de: De) -> bool;

pub open spec fn PayloadContainsLineForDe(array: [UInt32; 1], de: De) -> bool;

pub open spec fn PayloadContainsPrologue(array: [UInt32; 1]) -> bool;

pub open spec fn PayloadContainsEpilogue(array: [UInt32; 1]) -> bool;

} // verus!
