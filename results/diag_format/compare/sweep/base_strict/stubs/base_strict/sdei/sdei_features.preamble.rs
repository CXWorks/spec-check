use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type UInt32 = u32;

pub struct S {
    pub sdei_enabled: bool,
    pub shared_slots: int,
    pub private_slots: int,
}

pub spec const NOT_SUPPORTED: Int64 = (-1int) as i64;
pub spec const INVALID_PARAMETERS: Int64 = (-2int) as i64;

pub spec const BIND_SLOTS: UInt32 = 0;
pub spec const RELATIVE_MODE: UInt32 = 1;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn IsDefinedSdeiFeature(feature: UInt32) -> bool;

pub open spec fn Bits(value: Int64, hi: int, lo: int) -> int;

pub open spec fn SharedEventSlotCount() -> int;

pub open spec fn PrivateEventSlotCount() -> int;

pub open spec fn RelativeModeSupported() -> bool;

} // verus!
