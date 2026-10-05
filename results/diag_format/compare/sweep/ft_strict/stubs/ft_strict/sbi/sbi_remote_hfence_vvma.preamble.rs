use vstd::prelude::*;
verus! {

pub type UnsignedLong = u64;

pub type SbiError = i64;

pub type HartId = u64;

pub type Vmid = u64;

pub struct S {
    pub dummy: u64,
}

pub const SBI_SUCCESS: SbiError = 0;
pub const SBI_ERR_FAILED: SbiError = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiError = -2;
pub const SBI_ERR_INVALID_PARAM: SbiError = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiError = -5;

pub open spec fn RemoteHfenceVvmaImplemented() -> bool;

pub open spec fn IsTargetHart(h: HartId, hart_mask: u64, hart_mask_base: u64) -> bool;

pub open spec fn HartSupportsHypervisorExtension(h: HartId) -> bool;

pub open spec fn ResultEqual(a: SbiError, b: SbiError) -> bool;

pub open spec fn IsValidAddress(addr: u64) -> bool;

pub open spec fn IsValidSize(addr: u64, size: u64) -> bool;

pub open spec fn HartEnabledByPlatform(h: HartId) -> bool;

pub open spec fn HartAvailableToSupervisor(h: HartId) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason() -> bool;

pub open spec fn IpiSent(h: HartId) -> bool;

pub open spec fn HfenceVvmaExecuted(h: HartId, start: u64, end: int, vmid: Vmid) -> bool;

pub open spec fn HgatpVmid(h: HartId) -> Vmid;

pub open spec fn CallingHart() -> HartId;

} // verus!
