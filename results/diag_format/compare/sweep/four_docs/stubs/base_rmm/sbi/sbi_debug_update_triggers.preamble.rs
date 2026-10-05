use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct Tdata1 {
    pub r#type: u64,
    pub chain: bool,
}

pub struct TriggerConfig {
    pub trig_idx: u64,
    pub trig_tdata1: Tdata1,
    pub trig_tdata2: u64,
    pub trig_tdata3: u64,
}

pub struct Trigger {
    pub tdata1: Tdata1,
    pub tdata2: u64,
    pub tdata3: u64,
}

pub struct S {
    pub trig_count: u64,
    pub trig_configs: Seq<TriggerConfig>,
    pub triggers: Map<u64, Trigger>,
    pub installed: Set<u64>,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn trig_count(s: S) -> int;

pub open spec fn IsInstalledTrigger(idx: u64) -> bool;

pub open spec fn TrigConfig(s: S, i: int) -> TriggerConfig;

pub open spec fn InstalledTrigger(idx: u64) -> Trigger;

pub open spec fn InstalledTriggerIn(s: S, idx: u64) -> Trigger;

pub open spec fn TriggerUpdatedFrom(t: Trigger, cfg: TriggerConfig) -> bool;

pub open spec fn CommandFailed(result: sbiret) -> bool;

pub open spec fn CommandSucceeded(result: sbiret) -> bool;

} // verus!
