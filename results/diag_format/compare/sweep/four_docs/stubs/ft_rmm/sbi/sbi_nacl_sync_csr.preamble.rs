use vstd::prelude::*;
verus! {

pub type unsigned_long = u64;
pub type long = i64;

#[allow(non_camel_case_types)]
pub type unsigned = u32;

pub struct S {
    pub hcsr_synced: Map<u64, bool>,
    pub all_synced: bool,
}

pub open spec fn AllImplementedHCsrsSynchronized(s: S) -> bool;

pub open spec fn HCsrSynchronized(s: S, csr_num: u64) -> bool;

} // verus!
