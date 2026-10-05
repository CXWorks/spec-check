use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub const NOT_SUPPORTED: UInt32 = 0xFFFF_FFFFu32;

pub struct S {
    pub drtm_enabled: bool,
    pub version: UInt32,
}

} // verus!
