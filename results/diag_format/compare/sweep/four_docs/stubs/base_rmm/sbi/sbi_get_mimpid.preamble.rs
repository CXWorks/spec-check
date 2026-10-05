use vstd::prelude::*;

verus! {

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub mimpid: u64,
}

pub open spec fn IsLegalMimpidValue(value: sbiret) -> bool;

} // verus!
