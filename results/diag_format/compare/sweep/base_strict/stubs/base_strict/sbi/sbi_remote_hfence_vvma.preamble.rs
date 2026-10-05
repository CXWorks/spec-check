use vstd::prelude::*;

verus! {

pub type UInt = int;

pub type HartId = int;

pub type Vmid = int;

pub type SbiError = i64;

pub struct sbiret {
    pub error: SbiError,
    pub value: int,
}

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;

pub open spec fn RemoteHfenceVvmaImplemented() -> bool;

pub open spec fn ResultEqual(result: SbiError, code: i64) -> bool;

pub open spec fn IsTargetHart(h: HartId, hart_mask: UInt, hart_mask_base: UInt) -> bool;

pub open spec fn HartSupportsHypervisorExtension(h: HartId) -> bool;

pub open spec fn IsValidAddress(addr: UInt) -> bool;

pub open spec fn IsValidSize(start_addr: UInt, size: UInt) -> bool;

pub open spec fn HartEnabledByPlatform(h: HartId) -> bool;

pub open spec fn HartAvailableToSupervisor(h: HartId) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason() -> bool;

pub open spec fn IpiSent(h: HartId) -> bool;

pub open spec fn HfenceVvmaExecuted(h: HartId, start: int, end: int, vmid: Vmid) -> bool;

pub open spec fn HgatpVmid(h: HartId) -> Vmid;

pub open spec fn CallingHart() -> HartId;

} // verus!
