use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type SbiError = i64;

pub const SBI_SUCCESS: SbiError = 0;
pub const SBI_ERR_FAILED: SbiError = -1;
pub const SBI_ERR_INVALID_PARAM: SbiError = -3;

pub struct SbiRet {
    pub error: SbiError,
    pub value: i64,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn SbiHartMaskAllValid(s: S, hart_mask: UInt64, hart_mask_base: UInt64) -> bool;

pub open spec fn SbiRemoteFenceIIpiSentToAll(old_s: S, new_s: S, hart_mask: UInt64, hart_mask_base: UInt64) -> bool;

} // verus!
