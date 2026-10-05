use vstd::prelude::*;

verus! {

pub const NOT_SUPPORTED: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;

pub struct S {
    pub psci_func_id: u64,
    pub psci_version: u64,
}

} // verus!
