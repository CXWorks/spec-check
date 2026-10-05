use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub struct SbiRet {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub num_harts: u64,
}

pub spec const SBI_SUCCESS: i64 = 0;
pub spec const SBI_ERR_FAILED: i64 = -1 as i64;
pub spec const SBI_ERR_INVALID_PARAM: i64 = -3 as i64;

pub uninterp spec fn AllTargetHartsValid(s: S, hart_mask: UInt64, hart_mask_base: UInt64) -> bool;

pub uninterp spec fn SupervisorSoftwareInterruptPendingOnAllTargetHarts(old_s: S, new_s: S, hart_mask: UInt64, hart_mask_base: UInt64) -> bool;

} // verus!
