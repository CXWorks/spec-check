use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type Int64 = i64;
pub type Bits64 = u64;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: i64 = -1;

pub open spec fn IsImplementedDrtmFunctionOrFeature(s: S, id: UInt64) -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn FeatureId(id: UInt64) -> UInt64;

} // verus!
