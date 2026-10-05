use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub marchid: u64,
}

pub open spec fn IsLegalMarchidValue(value: sbiret) -> bool;

} // verus!
