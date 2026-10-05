use vstd::prelude::*;
verus! {

pub type unsigned_long = u64;

pub struct sbiret {
    pub ret: i64,
    pub value: i64,
}

pub struct S {
    pub num_harts: u64,
}

pub type HartId = u64;

pub type Vmid = u64;

pub open spec fn HartsSelectedBy(s: S, hart_mask: u64, hart_mask_base: u64) -> Set<HartId>;

pub open spec fn ExecutedHfenceVvma(s: S, hart: HartId, start_addr: u64, end_addr: int, asid: u64, vmid: Vmid) -> bool;

pub open spec fn CurrentVmid() -> Vmid;

} // verus!
