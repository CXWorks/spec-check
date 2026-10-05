use vstd::prelude::*;
verus! {

pub struct S {
    pub sdei_supported: bool,
    pub version: u64,
}

pub const NOT_SUPPORTED: i64 = -1i64;

pub open spec fn SdeiSupported(s: S) -> bool;

} // verus!
