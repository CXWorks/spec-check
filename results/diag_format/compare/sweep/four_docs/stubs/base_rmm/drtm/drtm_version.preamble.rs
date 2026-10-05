use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub drtm_supported: bool,
}

pub const NOT_SUPPORTED: UInt32 = 0xFFFF_FFFFu32;

pub open spec fn DrtmIsSupported() -> bool;

pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;

} // verus!
