use vstd::prelude::*;

verus! {

pub type Bits64 = u64;
pub type UInt64 = u64;
pub type PsciReturnCode = i64;
pub type PowerState = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const NOT_SUPPORTED: PsciReturnCode = (-1int) as i64;
pub spec const INVALID_PARAMETERS: PsciReturnCode = (-2int) as i64;
pub spec const HW_ON: PsciReturnCode = 0;
pub spec const HW_OFF: PsciReturnCode = 1;
pub spec const HW_STANDBY: PsciReturnCode = 2;

pub spec const RUN: PowerState = 0;
pub spec const POWERDOWN: PowerState = 1;
pub spec const STANDBY_OR_RETENTION: PowerState = 2;

pub uninterp spec fn NodeHwStateImplemented() -> bool;

pub uninterp spec fn IsValidNode(target_cpu: Bits64, power_level: UInt64) -> bool;

pub uninterp spec fn ResultEqual(result: PsciReturnCode, expected: PsciReturnCode) -> bool;

pub uninterp spec fn PowerControlView(target_cpu: Bits64, power_level: UInt64) -> PowerState;

} // verus!
