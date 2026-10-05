use vstd::prelude::*;
use vstd::arithmetic::power::pow;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;
pub type ReturnCode = i32;
pub type De = u64;

pub struct Control {
    pub reserved: u32,
    pub selector: u32,
    pub enable: u32,
    pub mode: u32,
}

pub struct SamplingRate {
    pub sec: u64,
    pub exponent: u32,
}

pub struct ProtocolAttributesInfo {
    pub attributes_1: Seq<u64>,
}

pub struct S {
    pub dummy: u64,
}

pub spec const RSI_SUCCESS: Int32 = 0;

pub spec const SUCCESS: ReturnCode = 0;
pub spec const INVALID_PARAMETERS: ReturnCode = -1;
pub spec const OUT_OF_RANGE: ReturnCode = -2;

pub spec const result: Int32 = 7;
pub spec const d: De = 0;

pub open spec fn ResultEqual(r: Int32, code: ReturnCode) -> bool;

pub open spec fn AreValidTelemetryConfigParams(s: S, group_identifier: UInt32, control: Control, sampling_rate: SamplingRate) -> bool;

pub open spec fn ProtocolAttributes(s: S) -> ProtocolAttributesInfo;

pub open spec fn NoDeEnabled(s: S, selector: u32, group_identifier: UInt32, control: Control) -> bool;

pub open spec fn EnabledDeOrGroupLimitReached(s: S, selector: u32, group_identifier: UInt32, mode: u32) -> bool;

pub open spec fn TargetDes(s: S, selector: u32, group_identifier: UInt32) -> Set<De>;

pub open spec fn TelemetryEnabled(s: S, de: De) -> u32;

pub open spec fn TelemetryMode(s: S, de: De) -> u32;

pub open spec fn InterfaceSupportsOnDemand(s: S, de: De) -> bool;

pub open spec fn SamplingRate(s: S, de: De) -> int;

} // verus!
