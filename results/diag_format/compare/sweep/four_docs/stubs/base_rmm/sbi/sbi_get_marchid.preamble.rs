use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type Int64 = i64;

pub struct sbiret {
    pub error: Int64,
    pub value: UInt64,
}

pub struct S {
    pub marchid: UInt64,
}

pub open spec fn IsLegalMarchidValue(v: UInt64) -> bool;

} // verus!
