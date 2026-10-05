use vstd::prelude::*;

verus! {

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub struct S {
    pub trig_max: u64,
    pub trig_idx_base: u64,
    pub trig_idx_mask: u64,
}

impl S {
    pub open spec fn trig_mapped(&self, trig_idx: u64) -> bool;
    pub open spec fn trig_vs(&self, trig_idx: u64) -> bool;
    pub open spec fn trig_vu(&self, trig_idx: u64) -> bool;
    pub open spec fn trig_s(&self, trig_idx: u64) -> bool;
    pub open spec fn trig_u(&self, trig_idx: u64) -> bool;
}

} // verus!
