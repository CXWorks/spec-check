use vstd::prelude::*;
verus! {

pub type UnsignedLong = u64;
pub type HartId = u64;
pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsHartSelected(hart_mask: UnsignedLong, hart_mask_base: UnsignedLong, h: HartId) -> bool;

pub open spec fn HartImplementsHypervisorExtension(s: S) -> bool;

pub open spec fn HfenceGvmaExecuted(s: S, h: HartId, start: UnsignedLong, end: int, vmid: UnsignedLong) -> bool;

pub open spec fn GStageTlbEntries(s: S, h: HartId, vmid: UnsignedLong, start: UnsignedLong, end: int) -> bool;

} // verus!
