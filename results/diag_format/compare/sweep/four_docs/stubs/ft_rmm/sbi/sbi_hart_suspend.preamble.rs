use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type unsigned_long = u64;
pub type HartId = u64;
pub type PrivilegeModeSet = u8;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub suspended_harts: Map<HartId, UInt32>,
    pub retentive_suspend_types: Set<UInt32>,
}

pub enum WakeupEvent {
    InterruptOrPlatformEvent,
    None,
}

pub spec const AllPrivilegeModes: PrivilegeModeSet = 7;

pub open spec fn CallingHart() -> HartId;

pub open spec fn InterruptOrPlatformEvent() -> WakeupEvent;

pub open spec fn HartIsSuspended(s: S, hart: HartId, suspend_type: UInt32) -> bool;

pub open spec fn HartResumesOn(s: S, hart: HartId, event: WakeupEvent) -> bool;

pub open spec fn IsRetentiveSuspendType(s: S, suspend_type: UInt32) -> bool;

pub open spec fn CallReturnsWithoutFailure(s: S, ret: sbiret) -> bool;

pub open spec fn HartRegistersAndCsrsPreserved(s: S, hart: HartId, modes: PrivilegeModeSet) -> bool;

pub open spec fn CallReturnsWithFailure(ret: sbiret) -> bool;

} // verus!
