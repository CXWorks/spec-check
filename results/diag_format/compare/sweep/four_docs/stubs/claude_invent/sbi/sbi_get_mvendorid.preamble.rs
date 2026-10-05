use vstd::prelude::*;

verus! {

pub struct S {
    pub mvendorid: u64,
}

pub open spec fn IsLegalMvendoridValue(s: S, value: u64) -> bool;

} // verus!
