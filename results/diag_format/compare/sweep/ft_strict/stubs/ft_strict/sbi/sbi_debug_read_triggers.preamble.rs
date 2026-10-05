use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type UnsignedLong = u64;

pub type HartId = u64;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
    pub ret: i64,
}

pub struct S {
    pub dummy: int,
}

pub const XLEN: u64 = 64;

pub open spec fn CallingHart() -> HartId;

pub open spec fn SharedMemoryHoldsTriggerStateLE(s: S, hart: HartId, trig_idx: int, offset: int, width: int) -> bool;

} // verus!
