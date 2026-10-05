use vstd::prelude::*;
verus! {

pub type UnsignedLong = u64;
pub type SbiError = i64;
pub type FunctionId = u64;
pub type Vmid = u64;
pub type HgatpValue = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const SBI_SUCCESS: SbiError = 0;
pub spec const SBI_ERR_FAILED: SbiError = -1;
pub spec const SBI_ERR_NOT_SUPPORTED: SbiError = -2;
pub spec const SBI_ERR_INVALID_PARAM: SbiError = -3;
pub spec const SBI_ERR_INVALID_ADDRESS: SbiError = -5;

pub spec const result: SbiError = 1;

pub spec const RemoteHfenceVvma: FunctionId = 6;

pub open spec fn IsFunctionImplemented(s: S, fid: FunctionId) -> bool;

pub open spec fn ResultEqual(a: SbiError, b: SbiError) -> bool;

pub open spec fn AllTargetHartsImplementHypervisorExt(s: S, hart_mask: UnsignedLong, hart_mask_base: UnsignedLong) -> bool;

pub open spec fn IsValidAddressRange(s: S, start_addr: UnsignedLong, size: UnsignedLong) -> bool;

pub open spec fn AllHartIdsValid(s: S, hart_mask: UnsignedLong, hart_mask_base: UnsignedLong) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(s: S) -> bool;

pub open spec fn IpiSentToAllTargetHarts(s: S, hart_mask: UnsignedLong, hart_mask_base: UnsignedLong) -> bool;

pub open spec fn TargetHartsExecutedHfenceVvma(s: S, hart_mask: UnsignedLong, hart_mask_base: UnsignedLong, start_addr: UnsignedLong, end_addr: int, vmid: Vmid) -> bool;

pub open spec fn hgatp(s: S) -> HgatpValue;

pub open spec fn CurrentVmid(h: HgatpValue) -> Vmid;

} // verus!
