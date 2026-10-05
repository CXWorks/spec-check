use vstd::prelude::*;

verus! {

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: int,
}

pub spec const SBI_SUCCESS: i64 = 0;
pub spec const SBI_ERR_INVALID_ADDRESS: i64 = -5;

pub open spec fn IsValidStartAddr(start_addr: u64) -> bool;

pub open spec fn IsValidSize(size: u64) -> bool;

pub open spec fn ResultEqual(result: sbiret, code: i64) -> bool;

pub open spec fn TargetHarts(hart_mask: u64, hart_mask_base: u64) -> Set<u64>;

pub open spec fn IpiSentToAll(harts: Set<u64>) -> bool;

pub open spec fn SfenceVmaExecuted(hart: u64, start: u64, end: int) -> bool;

} // verus!
