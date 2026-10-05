use vstd::prelude::*;

verus! {

pub type unsigned = u64;
pub type long = u64;
pub type UInt64 = u64;

pub type SbiCommandReturnCode = i64;

pub spec const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub spec const SBI_ERR_FAILED: SbiCommandReturnCode = -1;
pub spec const SBI_ERR_NOT_SUPPORTED: SbiCommandReturnCode = -2;
pub spec const SBI_ERR_INVALID_PARAM: SbiCommandReturnCode = -3;
pub spec const SBI_ERR_DENIED: SbiCommandReturnCode = -4;

pub spec const i: u64 = 0;

pub struct Tdata1 {
    pub r#type: u64,
    pub chain: u64,
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
}

pub open spec fn Exists(var: u64, range: bool, cond: bool) -> bool;

pub open spec fn ForAll(var: u64, range: bool, cond: bool) -> bool;

pub open spec fn TrigConfig(idx: u64) -> TriggerConfig;

pub open spec fn IsInstalledTrigger(trig_idx: u64) -> bool;

pub open spec fn InstalledTrigger(trig_idx: u64) -> Trigger;

pub open spec fn CommandFailed(result: SbiCommandReturnCode) -> bool;

} // verus!
