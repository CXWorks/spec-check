use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: u64,
}

pub spec const hart_mask: u64 = 1;
pub spec const hart_mask_base: u64 = 2;
pub spec const start_addr: u64 = 3;
pub spec const size: u64 = 4;
pub spec const vmid: u64 = 5;

pub uninterp spec fn HfenceGvmaExecutedOnHarts(s: S, hm: u64, hmb: u64, sa: u64, sz: u64, vm: u64) -> bool;

} // verus!
