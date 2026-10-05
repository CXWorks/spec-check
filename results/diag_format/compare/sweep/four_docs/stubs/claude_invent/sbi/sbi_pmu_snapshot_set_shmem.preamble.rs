use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct SbiRet {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub calling_hart: UInt64,
    pub dummy: int,
}

pub open spec fn SbiRetIsSuccess(r: SbiRet) -> bool;

pub open spec fn CallingHart(s: S) -> UInt64;

pub open spec fn PmuSnapshotEnabled(s: S, hart: UInt64) -> bool;

pub open spec fn PmuSnapshotShmemBase(s: S, hart: UInt64) -> int;

pub open spec fn PmuSnapshotShmemSize(s: S, hart: UInt64) -> int;

pub open spec fn PhysAddrFromParts(lo: UInt64, hi: UInt64) -> int;

} // verus!
