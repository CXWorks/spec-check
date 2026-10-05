use vstd::prelude::*;

verus! {

pub struct Int64 {
    pub v: i64,
}

impl Int64 {
    pub open spec fn spec_index(self, i: int) -> int;
}

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: Int64 = Int64 { v: -1i64 };

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn VendorDefinedVersion() -> int;

pub open spec fn SdeiImplementsAllCalls() -> bool;

} // verus!
