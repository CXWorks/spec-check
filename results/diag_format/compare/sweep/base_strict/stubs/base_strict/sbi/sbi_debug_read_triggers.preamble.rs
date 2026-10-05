use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type RsiCommandReturnCode = u64;

pub type HartId = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;

pub const XLEN: UInt64 = 64;

pub struct S {
    pub trig_count: UInt64,
    pub trig_idx_base: UInt64,
}

pub open spec fn trig_count(s: S) -> UInt64;

pub open spec fn trig_idx_base(s: S) -> UInt64;

pub open spec fn CallingHart() -> HartId;

pub open spec fn SharedMemoryHoldsTriggerStateLE(hart: HartId, trig_idx: int, offset: int, size: int) -> bool;

} // verus!
