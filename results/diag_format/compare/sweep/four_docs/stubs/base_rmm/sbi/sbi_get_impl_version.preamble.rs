use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn CurrentSbiImplVersion() -> u64;

} // verus!
