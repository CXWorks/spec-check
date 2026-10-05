use vstd::prelude::*;

verus! {

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub const TRIG_STATE_UNMAPPED: u64 = 0;
pub const TRIG_STATE_MAPPED: u64 = 1;

pub const trig_idx_base: u64 = 2;
pub const trig_idx_mask: u64 = 63;
pub const trig_max: u64 = 32;

pub struct S {
    pub dummy: u64,
}

impl S {
    pub open spec fn trig_state(self, idx: u64) -> u64;

    pub open spec fn tdata1(self, idx: u64) -> u64;

    pub open spec fn tdata2(self, idx: u64) -> u64;

    pub open spec fn tdata3(self, idx: u64) -> u64;
}

} // verus!
