use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int64 = i64;

pub struct S {
    pub sdei_supported: bool,
    pub relative_mode_supported: bool,
    pub shared_event_slots: Int64,
    pub private_event_slots: Int64,
}

pub const SDEI_SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;

pub const BIND_SLOTS: UInt32 = 0;
pub const RELATIVE_MODE: UInt32 = 1;

pub open spec fn IsSdeiSupported(s: S) -> bool;

pub open spec fn IsValidSdeiFeature(s: S, feature: UInt32) -> bool;

pub open spec fn IsRelativeModeSupported(s: S) -> bool;

pub open spec fn ResultEqual(result: Int64, expected: Int64) -> bool;

pub open spec fn SharedEventSlotCount(s: S) -> Int64;

pub open spec fn PrivateEventSlotCount(s: S) -> Int64;

} // verus!
