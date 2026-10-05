use vstd::prelude::*;
verus! {

pub type Int64 = i64;
pub type UInt64 = u64;

pub struct S {
    pub marchid: UInt64,
    pub mvendorid: UInt64,
    pub mimpid: UInt64,
}

pub open spec fn IsLegalMarchidValue(value: UInt64) -> bool;

} // verus!
