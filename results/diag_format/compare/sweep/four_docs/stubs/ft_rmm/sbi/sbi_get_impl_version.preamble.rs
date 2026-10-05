use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type Int64 = i64;

pub struct sbiret {
    pub error: Int64,
    pub value: UInt64,
}

pub struct S {
    pub sbi_impl_version: UInt64,
}

pub open spec fn CurrentSbiImplVersion() -> UInt64;

} // verus!
