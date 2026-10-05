use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct S {
    pub mimpid: u64,
}

pub open spec fn IsLegalMimpidCsrValue(s: S, value: UInt64) -> bool;

} // verus!
