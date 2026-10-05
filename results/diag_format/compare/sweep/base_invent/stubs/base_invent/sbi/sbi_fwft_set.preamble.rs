use vstd::prelude::*;

verus! {

pub struct sbiret {
    pub code: i64,
    pub value: i64,
}

pub struct S {
    pub firmware_features: u64,
}

} // verus!
