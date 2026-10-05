use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

impl sbiret {
    pub open spec fn is_Ok(self) -> bool;
}

pub struct S {
    pub dummy: u64,
}

pub spec const SBI_ERROR_INVALID_ARGS: i64 = (-3int) as i64;

#[allow(non_upper_case_globals)]
pub spec const hart_mask: u64 = 11;
#[allow(non_upper_case_globals)]
pub spec const hart_mask_base: u64 = 12;
#[allow(non_upper_case_globals)]
pub spec const start_addr: u64 = 13;
#[allow(non_upper_case_globals)]
pub spec const size: u64 = 14;
#[allow(non_upper_case_globals)]
pub spec const vmid: u64 = 15;

pub open spec fn ResultEqual(result: sbiret, code: i64) -> bool;

} // verus!
