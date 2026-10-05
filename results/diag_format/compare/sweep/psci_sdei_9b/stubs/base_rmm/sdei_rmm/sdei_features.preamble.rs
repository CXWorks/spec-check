use vstd::prelude::*;
verus! {

pub type Int64 = i64;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;

pub const BIND_SLOTS: UInt32 = 0;
pub const RELATIVE_MODE: UInt32 = 1;

pub open spec fn IsSdeiSupported() -> bool;

pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;

pub open spec fn IsValidSdeiFeature(feature: UInt32) -> bool;

pub open spec fn SharedEventSlotCount() -> Int64;

pub open spec fn PrivateEventSlotCount() -> Int64;

pub open spec fn IsRelativeModeSupported() -> bool;

} // verus!
