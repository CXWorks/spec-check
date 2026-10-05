use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int64 = i64;
pub type UInt64 = u64;

pub struct S {
    pub sdei_enabled: bool,
    pub state_word: u64,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;

pub const BIND_SLOTS: UInt32 = 0;
pub const RELATIVE_MODE: UInt32 = 1;

pub open spec fn IsSdeiSupported() -> bool;

pub open spec fn IsValidSdeiFeature(feature: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;

pub open spec fn SharedEventSlotCount() -> u32;

pub open spec fn PrivateEventSlotCount() -> u32;

pub open spec fn IsRelativeModeSupported() -> bool;

} // verus!
