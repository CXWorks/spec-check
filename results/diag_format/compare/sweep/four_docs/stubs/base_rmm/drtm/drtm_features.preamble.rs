use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub struct UInt64 {
    pub value: u64,
}

impl UInt64 {
    pub open spec fn spec_index(self, i: int) -> int;
}

pub struct Bits64 {
    pub value: u64,
}

impl Bits64 {
    pub open spec fn spec_index(self, i: int) -> int;
}

pub struct S {
    pub drtm_state: u64,
}

pub const NOT_SUPPORTED: Int64 = -1;

pub open spec fn IsImplementedDrtmFunctionOrFeature(id: UInt64) -> bool;

pub open spec fn ResultEqual(result: Int64, expected: Int64) -> bool;

pub open spec fn FeatureId(id: UInt64) -> int;

} // verus!
