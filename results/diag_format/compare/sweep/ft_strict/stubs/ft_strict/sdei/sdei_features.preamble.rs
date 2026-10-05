use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int64 = i64;

pub struct S {
    pub dummy: int,
}

pub const SDEI_SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;

pub const BIND_SLOTS: UInt32 = 0;
pub const RELATIVE_MODE: UInt32 = 1;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn ResultEqual(result: Int64, expected: Int64) -> bool;
pub open spec fn IsDefinedSdeiFeature(s: S, feature: UInt32) -> bool;
pub open spec fn Bits(value: Int64, hi: int, lo: int) -> int;
pub open spec fn SharedEventSlotCount(s: S) -> int;
pub open spec fn PrivateEventSlotCount(s: S) -> int;
pub open spec fn RelativeModeSupported(s: S) -> bool;

} // verus!
