use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int64 = i64;
pub type UInt64 = u64;

pub struct S {
    pub sdei_state: u64,
}

pub const NOT_SUPPORTED: Int64 = -1i64;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn VendorDefinedVersion() -> UInt32;

pub open spec fn SdeiImplementsAllCalls() -> bool;

} // verus!
