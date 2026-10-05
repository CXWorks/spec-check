use vstd::prelude::*;

verus! {

pub type uint32_t = u32;

pub type unsigned_long = u64;

pub struct sbiret {
    pub code: i64,
    pub value: i64,
}

pub type HartId = u64;

pub struct S {
    pub current_hart: HartId,
}

pub open spec fn CallingHart() -> HartId;

pub open spec fn HartEnteredSuspendState(s: S, hart: HartId, suspend_type: uint32_t) -> bool;

pub open spec fn HartResumesOnInterruptOrPlatformEvent(s: S, hart: HartId) -> bool;

pub open spec fn IsRetentiveSuspendType(s: S, suspend_type: uint32_t) -> bool;

pub open spec fn HartRegistersAndCsrsPreserved(s: S, hart: HartId) -> bool;

pub open spec fn CallReturnsWithoutFailure(s: S, result: sbiret) -> bool;

pub open spec fn ResumeAddrUnused(s: S, resume_addr: unsigned_long) -> bool;

} // verus!
