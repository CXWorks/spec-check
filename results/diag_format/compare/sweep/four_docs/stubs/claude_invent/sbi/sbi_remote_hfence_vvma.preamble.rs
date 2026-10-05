use vstd::prelude::*;

verus! {

pub type UInt64 = u64;
pub type Int64 = i64;

pub struct SbiRet {
    pub error: Int64,
    pub value: Int64,
}

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: Int64 = 0;
pub const SBI_ERR_FAILED: Int64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: Int64 = -2;
pub const SBI_ERR_INVALID_PARAM: Int64 = -3;
pub const SBI_ERR_INVALID_ADDRESS: Int64 = -5;

pub open spec fn SbiRemoteHfenceVvmaImplemented(s: S) -> bool;

pub open spec fn AllTargetHartsSupportHypervisorExtension(s: S, hart_mask: UInt64, hart_mask_base: UInt64) -> bool;

pub open spec fn IsValidGuestVirtualAddressRange(s: S, start_addr: UInt64, size: UInt64) -> bool;

pub open spec fn AllTargetHartIdsValid(s: S, hart_mask: UInt64, hart_mask_base: UInt64) -> bool;

pub open spec fn CallingHartHgatpVmid(s: S) -> UInt64;

pub open spec fn HfenceVvmaIpiSentToAllTargetHarts(old_s: S, new_s: S, hart_mask: UInt64, hart_mask_base: UInt64, start_addr: UInt64, size: UInt64, vmid: UInt64) -> bool;

} // verus!
