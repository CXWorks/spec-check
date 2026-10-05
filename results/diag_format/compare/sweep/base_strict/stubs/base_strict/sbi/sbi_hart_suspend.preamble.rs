use vstd::prelude::*;
verus! {

pub type HartId = u64;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn CallingHart() -> HartId;

pub open spec fn HartEnteredSuspendState(hart: HartId, suspend_type: u32) -> bool;

pub open spec fn HartResumesOnInterruptOrPlatformEvent(hart: HartId) -> bool;

pub open spec fn IsRetentiveSuspendType(suspend_type: u32) -> bool;

pub open spec fn HartRegistersAndCsrsPreserved(hart: HartId) -> bool;

pub open spec fn CallReturnsWithoutFailure(result: sbiret) -> bool;

pub open spec fn ResumeAddrUnused(resume_addr: u64) -> bool;

} // verus!
