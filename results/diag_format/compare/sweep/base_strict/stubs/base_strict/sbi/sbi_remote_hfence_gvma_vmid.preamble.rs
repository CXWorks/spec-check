use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type HartId = u64;

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsHartSelected(hart_mask: UInt64, hart_mask_base: UInt64, h: HartId) -> bool;

pub open spec fn HartImplementsHypervisorExtension(h: HartId) -> bool;

pub open spec fn HfenceGvmaExecuted(h: HartId, start: UInt64, end: int, vmid: UInt64) -> bool;

pub open spec fn GStageTlbEntries(h: HartId, vmid: UInt64, start: UInt64, end: int) -> bool;

} // verus!
