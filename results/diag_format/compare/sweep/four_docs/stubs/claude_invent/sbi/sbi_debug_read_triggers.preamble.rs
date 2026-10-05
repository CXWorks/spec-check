use vstd::prelude::*;
verus! {

pub type HartId = u64;

pub type SbiError = i64;

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_DENIED: i64 = -4;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: i64 = -6;
pub const SBI_ERR_NO_SHMEM: i64 = -9;

pub struct SbiRet {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub xlen: u64,
    pub calling_hart: HartId,
}

pub struct TriggerStateAndConfig {
    pub tstate: u64,
    pub tdata1: u64,
    pub tdata2: u64,
    pub tdata3: u64,
}

pub open spec fn Xlen(s: S) -> u64;

pub open spec fn CallingHart(s: S) -> HartId;

pub open spec fn DebugTriggerStateAndConfig(s: S, hart: HartId, trig_idx: int) -> TriggerStateAndConfig;

pub open spec fn ShmemDebugTriggerEntryEqual(s: S, offset: int, entry: TriggerStateAndConfig) -> bool;

pub open spec fn DebugTriggersUnchanged(old_s: S, new_s: S, hart: HartId) -> bool;

} // verus!
