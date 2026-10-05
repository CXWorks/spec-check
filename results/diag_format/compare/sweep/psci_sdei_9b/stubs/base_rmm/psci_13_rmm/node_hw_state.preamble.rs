use vstd::prelude::*;

verus! {

pub type RsiCommandReturnCode = u64;
pub type PowerState = u64;
pub type CpuId = u64;
pub type PowerLevel = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const NOT_SUPPORTED: RsiCommandReturnCode = 1;
pub spec const INVALID_PARAMETERS: RsiCommandReturnCode = 2;
pub spec const HW_ON: RsiCommandReturnCode = 3;
pub spec const HW_OFF: RsiCommandReturnCode = 4;
pub spec const HW_STANDBY: RsiCommandReturnCode = 5;

pub spec const RUN: PowerState = 10;
pub spec const POWERDOWN: PowerState = 11;
pub spec const STANDBY_OR_RETENTION: PowerState = 12;

pub spec const target_cpu: CpuId = 20;
pub spec const power_level: PowerLevel = 21;

pub open spec fn NodeHwStateImplemented() -> bool;

pub open spec fn ResultEqual(result: RsiCommandReturnCode, code: RsiCommandReturnCode) -> bool;

pub open spec fn IsValidNode(s: S, cpu: CpuId, level: PowerLevel) -> bool;

pub open spec fn PowerControlView(s: S, cpu: CpuId, level: PowerLevel) -> PowerState;

} // verus!
