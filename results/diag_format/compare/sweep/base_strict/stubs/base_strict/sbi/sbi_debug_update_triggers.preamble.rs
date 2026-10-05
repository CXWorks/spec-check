use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub shmem_base: UInt64,
    pub hart_id: UInt64,
}

pub struct DebugTrigger {
    pub r#type: UInt64,
    pub chain: bool,
}

pub open spec fn trig_count(s: S) -> UInt64;

pub open spec fn ShmemTrigIdx(i: UInt64, s: S) -> UInt64;

pub open spec fn ShmemTrigTdata1(i: UInt64, s: S) -> UInt64;

pub open spec fn ShmemTrigTdata2(i: UInt64, s: S) -> UInt64;

pub open spec fn ShmemTrigTdata3(i: UInt64, s: S) -> UInt64;

pub open spec fn IsInstalledDebugTrigger(idx: UInt64) -> bool;

pub open spec fn InstalledDebugTrigger(idx: UInt64) -> DebugTrigger;

pub open spec fn TriggerType(tdata1: UInt64) -> UInt64;

pub open spec fn TriggerChain(tdata1: UInt64) -> bool;

pub open spec fn DebugTriggerUpdatedFrom(idx: UInt64, tdata1: UInt64, tdata2: UInt64, tdata3: UInt64) -> bool;

pub open spec fn CallFails(result: sbiret) -> bool;

pub open spec fn CallSucceeds(result: sbiret) -> bool;

} // verus!
