use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

impl sbiret {
    pub open spec fn is_Ok(self) -> bool;
}

pub const SBI_ERROR_INVALID_ARGS: i64 = -3;

pub const hart_mask: u64 = 11;
pub const hart_mask_base: u64 = 12;
pub const start_addr: u64 = 13;
pub const size: u64 = 14;
pub const asid: u64 = 15;

pub open spec fn ResultEqual(result: sbiret, code: i64) -> bool;

} // verus!
