use vstd::prelude::*;

verus! {

pub struct SbiRet {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: nat,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;

pub open spec fn IsValidSfenceVmaAddressRange(s: S, start_addr: u64, size: u64) -> bool;

pub open spec fn RemoteSfenceVmaSentToTargetHarts(old_s: S, new_s: S, hart_mask: u64, hart_mask_base: u64, start_addr: u64, size: u64) -> bool;

} // verus!
