use vstd::prelude::*;

verus! {

pub type int32 = i32;

pub struct S {
    pub version: u32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;

pub open spec fn version_not_supported(old_s: S, result: int32) -> bool;

pub open spec fn version_supported(old_s: S, result: int32) -> bool;

} // verus!
