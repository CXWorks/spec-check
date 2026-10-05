use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct S {
    pub marchid: UInt64,
}

pub open spec fn IsLegalMarchidValue(value: UInt64) -> bool;

} // verus!
