use vstd::prelude::*;
verus! {

pub type UnsignedLong = u64;
pub type unsigned_long = u64;
pub type HartId = u64;
pub type PhysAddr = int;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
    pub code: i64,
}

pub struct MemRegion {
    pub base: PhysAddr,
    pub size: int,
}

pub struct S {
    pub dummy: int,
}

pub spec const calling_hart: HartId = 0;

pub open spec fn IsAllOnes(s: S, x: u64) -> bool;

pub open spec fn StealTimeShmemBase(s: S, hart: HartId) -> PhysAddr;

pub open spec fn Concat(s: S, hi: u64, lo: u64) -> PhysAddr;

pub open spec fn Mem(base: PhysAddr, size: int) -> MemRegion;

pub open spec fn IsZero(s: S, region: MemRegion) -> bool;

pub open spec fn StealTimeReportingEnabled(s: S, hart: HartId) -> bool;

} // verus!
