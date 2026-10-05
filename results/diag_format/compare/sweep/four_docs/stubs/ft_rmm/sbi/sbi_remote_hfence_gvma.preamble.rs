use vstd::prelude::*;
verus! {

#[allow(non_camel_case_types)]
pub type unsigned_long = u64;

pub type SbiErrorCode = i64;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;

pub const SBI_REMOTE_HFENCE_GVMA: u64 = 3;

pub const ALL_GUESTS: u64 = 0xFFFF_FFFF_FFFF_FFFF;

pub open spec fn IsFunctionImplemented(s: S, fid: u64) -> bool;

pub open spec fn AnyTargetHartLacksHypervisorExtension(s: S, hart_mask: u64, hart_mask_base: u64) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn IsValidAddressRange(s: S, start_addr: u64, size: u64) -> bool;

pub open spec fn AnyTargetHartIdInvalid(s: S, hart_mask: u64, hart_mask_base: u64) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(s: S) -> bool;

pub open spec fn IpiSentToAllTargetHarts(s: S, hart_mask: u64, hart_mask_base: u64) -> bool;

pub open spec fn TargetHartsInstructedToExecuteHfenceGvma(s: S, hart_mask: u64, hart_mask_base: u64, start_addr: u64, end_addr: int, vmid: u64) -> bool;

} // verus!
