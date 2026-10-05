use vstd::prelude::*;

verus! {

pub type SbiErrorCode = i64;

pub struct S {
    pub implemented_functions: Set<u64>,
    pub harts: Set<int>,
}

pub spec const SBI_SUCCESS: SbiErrorCode = 0;
pub spec const SBI_ERR_FAILED: SbiErrorCode = (-1int) as i64;
pub spec const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = (-2int) as i64;
pub spec const SBI_ERR_INVALID_PARAM: SbiErrorCode = (-3int) as i64;
pub spec const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = (-5int) as i64;

pub spec const SBI_REMOTE_HFENCE_GVMA: u64 = 2;

pub spec const ALL_GUESTS: u64 = 0xFFFF_FFFF;

pub spec const hart_mask: u64 = 1;
pub spec const hart_mask_base: u64 = 2;

pub spec const start_addr: int = 4096;
pub spec const size: int = 8192;

pub open spec fn IsFunctionImplemented(s: S, fid: u64) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn AnyTargetHartLacksHypervisorExtension(s: S, mask: u64, mask_base: u64) -> bool;

pub open spec fn IsValidAddressRange(start: int, sz: int) -> bool;

pub open spec fn AnyTargetHartIdInvalid(s: S, mask: u64, mask_base: u64) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason() -> bool;

pub open spec fn IpiSentToAllTargetHarts(mask: u64, mask_base: u64) -> bool;

pub open spec fn TargetHartsInstructedToExecuteHfenceGvma(mask: u64, mask_base: u64, start: int, end: int, vmid: u64) -> bool;

pub open spec fn TargetHarts(mask: u64, mask_base: u64) -> Set<int>;

pub open spec fn GuestPhysicalTranslations(s: S, harts: Set<int>, start: int, end: int, vmid: u64) -> bool;

} // verus!
