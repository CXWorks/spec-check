use vstd::prelude::*;
verus! {

pub struct S {
    pub hart_mask: int,
    pub hart_mask_base: int,
    pub start_addr: int,
    pub size: int,
    pub asid: int,
    pub vmid: int,
}

pub open spec fn hart_mask(s: S) -> int;

pub open spec fn hart_mask_base(s: S) -> int;

pub open spec fn start_addr(s: S) -> int;

pub open spec fn size(s: S) -> int;

pub open spec fn asid(s: S) -> int;

pub open spec fn CurrentVmid() -> int;

pub open spec fn RemoteHartsExecutedHfenceVvma(hart_mask: int, hart_mask_base: int, start: int, end: int, asid: int, vmid: int) -> bool;

} // verus!
