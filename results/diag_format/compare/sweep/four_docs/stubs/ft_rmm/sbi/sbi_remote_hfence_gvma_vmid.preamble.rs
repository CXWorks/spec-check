use vstd::prelude::*;

verus! {

pub type unsigned_long = u64;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn HfenceGvmaExecutedOnHarts(s: S, hart_mask: unsigned_long, hart_mask_base: unsigned_long, start_addr: unsigned_long, size: unsigned_long, vmid: unsigned_long) -> bool;

} // verus!
