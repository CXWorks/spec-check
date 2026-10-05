use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type Int64 = i64;

#[allow(non_camel_case_types)]
pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub mimpid: u64,
}

pub open spec fn IsLegalMimpidValue(value: sbiret) -> bool;

} // verus!
