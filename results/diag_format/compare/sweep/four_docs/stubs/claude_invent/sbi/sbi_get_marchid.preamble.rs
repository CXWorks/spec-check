use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct S {
    pub marchid: u64,
}

pub open spec fn IsLegalMarchidValue(s: S, value: UInt64) -> bool;

} // verus!
