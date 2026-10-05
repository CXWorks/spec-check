use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub protocol_version_max: uint32,
}

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_SUPPORTED: int32 = 1;

pub spec const version: uint32 = 2;

pub uninterp spec fn ResultEqual(a: int32, b: int32) -> bool;

pub uninterp spec fn version_is_supported(old_s: S, v: uint32) -> bool;

} // verus!
