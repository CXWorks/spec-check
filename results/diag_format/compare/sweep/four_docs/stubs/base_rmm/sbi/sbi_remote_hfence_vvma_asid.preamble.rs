use vstd::prelude::*;
verus! {

pub type Hart = nat;

pub struct sbiret {
    pub error: int,
    pub value: int,
    pub ret: int,
}

pub struct S {
    pub harts: Set<Hart>,
    pub vmid: int,
}

pub spec const hart_mask: int = 1;
pub spec const hart_mask_base: int = 2;
pub spec const start_addr: int = 4096;
pub spec const size: int = 8192;
pub spec const asid: int = 3;

pub open spec fn HartsSelectedBy(s: S, mask: int, mask_base: int) -> Set<Hart>;

pub open spec fn ExecutedHfenceVvma(hart: Hart, start: int, end: int, asid: int, vmid: int) -> bool;

pub open spec fn CurrentVmid() -> int;

pub open spec fn ForAll(b: bool) -> bool;

} // verus!
