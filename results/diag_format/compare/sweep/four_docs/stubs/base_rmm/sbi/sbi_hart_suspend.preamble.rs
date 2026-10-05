use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type HartId = u64;
pub type SuspendType = u32;
pub type PrivilegeModeSet = u64;

pub struct sbiret {
    pub error: i64,
    pub ret: i64,
}

pub struct S {
    pub suspend_type: SuspendType,
}

pub spec const AllPrivilegeModes: PrivilegeModeSet = 7;

pub open spec fn CallingHart() -> HartId;

pub open spec fn HartIsSuspended(s: S, hart: HartId, suspend_type: SuspendType) -> bool;

pub open spec fn HartResumesOn(s: S, hart: HartId, suspend_type: SuspendType) -> bool;

pub open spec fn IsRetentiveSuspendType(suspend_type: SuspendType) -> bool;

pub open spec fn HartRegistersAndCsrsPreserved(s: S, hart: HartId, modes: PrivilegeModeSet) -> bool;

} // verus!
