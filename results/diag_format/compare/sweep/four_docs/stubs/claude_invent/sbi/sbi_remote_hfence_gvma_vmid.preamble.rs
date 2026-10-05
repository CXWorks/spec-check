use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;

pub struct SbiRet {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn SbiRfenceExtensionImplemented(s: S) -> bool;

pub open spec fn TargetHartsImplementHypervisorExtension(s: S, hart_mask: UInt64, hart_mask_base: UInt64) -> bool;

pub open spec fn HartMaskIsValid(s: S, hart_mask: UInt64, hart_mask_base: UInt64) -> bool;

pub open spec fn GuestPhysAddrRangeIsValid(s: S, start_addr: UInt64, size: UInt64) -> bool;

pub open spec fn RemoteHfenceGvmaVmidExecuted(old_s: S, new_s: S, hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, vmid: UInt64) -> bool;

} // verus!
