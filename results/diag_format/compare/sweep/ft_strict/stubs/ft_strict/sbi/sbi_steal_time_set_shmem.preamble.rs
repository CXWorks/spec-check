use vstd::prelude::*;
verus! {

pub type UInt = u64;
pub type UInt64 = u64;
pub type HartId = u64;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsAllOnes(x: UInt) -> bool;
pub open spec fn CallingHart(s: S) -> HartId;
pub open spec fn StealTimeShmemBase(s: S, hart: HartId) -> UInt64;
pub open spec fn ShmemPhysAddr(s: S, lo: UInt, hi: UInt) -> UInt64;
pub open spec fn StealTimeReportingEnabled(s: S, hart: HartId) -> bool;
pub open spec fn MemByte(s: S, addr: int) -> u8;

} // verus!
