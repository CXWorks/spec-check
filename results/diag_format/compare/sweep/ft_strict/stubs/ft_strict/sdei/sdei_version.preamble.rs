use vstd::prelude::*;
verus! {

pub type Int64 = i64;

pub struct S {
    pub sdei_supported: bool,
}

pub const NOT_SUPPORTED: Int64 = -1;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn Bits(x: Int64, hi: int, lo: int) -> int;

pub open spec fn VendorDefinedVersion() -> int;

pub open spec fn SdeiImplementsAllCalls() -> bool;

} // verus!
