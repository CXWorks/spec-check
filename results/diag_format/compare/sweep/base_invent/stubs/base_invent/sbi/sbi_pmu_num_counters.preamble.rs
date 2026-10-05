use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: u64,
}

pub spec const SBI_SUCCESS: i64 = 0;

} // verus!
