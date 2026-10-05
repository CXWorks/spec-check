use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type unsigned_long = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub type SbiRet = sbiret;

pub struct DebugTrigger {
    pub r#type: u64,
    pub chain: u64,
    pub tdata1: u64,
    pub tdata2: u64,
    pub tdata3: u64,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsInstalledDebugTrigger(s: S, idx: UInt64) -> bool;

pub open spec fn ShmemTrigIdx(i: UInt64) -> UInt64;

pub open spec fn ShmemTrigTdata1(i: UInt64) -> UInt64;

pub open spec fn ShmemTrigTdata2(i: UInt64) -> UInt64;

pub open spec fn ShmemTrigTdata3(i: UInt64) -> UInt64;

pub open spec fn TriggerType(s: S, tdata1: UInt64) -> u64;

pub open spec fn TriggerChain(s: S, tdata1: UInt64) -> u64;

pub open spec fn InstalledDebugTrigger(s: S, idx: UInt64) -> DebugTrigger;

pub open spec fn DebugTriggerUpdatedFrom(s: S, idx: UInt64, tdata1: UInt64, tdata2: UInt64, tdata3: UInt64) -> bool;

pub open spec fn CallSucceeds(result: sbiret) -> bool;

pub open spec fn CallFails(result: sbiret) -> bool;

} // verus!
