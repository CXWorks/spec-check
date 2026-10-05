use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type SbiError = i64;

pub const SBI_SUCCESS: SbiError = 0;
pub const SBI_ERR_FAILED: SbiError = -1;
pub const SBI_ERR_INVALID_PARAM: SbiError = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiError = -5;

pub struct SbiRet {
    pub error: SbiError,
    pub value: i64,
}

pub struct S {
    pub num_harts: nat,
    pub max_asid: nat,
    pub tlb_flush_count: nat,
}

pub open spec fn SfenceAddrRangeValid(s: S, start_addr: UInt64, size: UInt64) -> bool;

pub open spec fn SfenceAsidValid(s: S, asid: UInt64) -> bool;

pub open spec fn HartMaskAllValid(s: S, hart_mask: UInt64, hart_mask_base: UInt64) -> bool;

pub open spec fn RemoteSfenceVmaAsidIssued(old_s: S, new_s: S, hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, asid: UInt64) -> bool;

} // verus!
