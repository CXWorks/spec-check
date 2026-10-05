use vstd::prelude::*;

verus! {

#[allow(non_camel_case_types)]
pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub hgatp: u64,
}

pub type SbiFunctionId = u64;

#[allow(non_upper_case_globals)]
pub const RemoteHfenceVvma: SbiFunctionId = 5;

#[allow(non_upper_case_globals)]
pub const hgatp: u64 = 0;

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;

pub open spec fn IsFunctionImplemented(f: SbiFunctionId) -> bool;

pub open spec fn ResultEqual(r: sbiret, code: i64) -> bool;

pub open spec fn AllTargetHartsImplementHypervisorExt(hart_mask: u64, hart_mask_base: u64) -> bool;

pub open spec fn IsValidAddressRange(start_addr: u64, size: u64) -> bool;

pub open spec fn AllHartIdsValid(hart_mask: u64, hart_mask_base: u64) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason() -> bool;

pub open spec fn IpiSentToAllTargetHarts(hart_mask: u64, hart_mask_base: u64) -> bool;

pub open spec fn TargetHartsExecutedHfenceVvma(hart_mask: u64, hart_mask_base: u64, start: u64, end: int, vmid: u64) -> bool;

pub open spec fn CurrentVmid(hgatp_val: u64) -> u64;

} // verus!
