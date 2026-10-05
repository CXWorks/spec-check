use vstd::prelude::*;
verus! {

pub type int64 = i64;
pub type PeId = u64;

pub const NOT_SUPPORTED: int64 = -1;

pub struct S {
    pub pe_id: PeId,
}

impl S {
    pub open spec fn sdei_pe_masked(self, pe: PeId) -> bool;
}

} // verus!
