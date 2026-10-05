use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub voltage_levels: Map<u32, i32>,
    pub domain_count: u32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2) as i32;
pub spec const DENIED: Int32 = (-3) as i32;
pub spec const NOT_FOUND: Int32 = (-4) as i32;

pub spec const domain_id: UInt32 = 1;
pub spec const calling_agent: UInt32 = 2;
pub spec const flags: UInt64 = 3;
pub spec const voltage_level: Int32 = 5;

pub uninterp spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub uninterp spec fn VoltageDomainExists(s: S, domain: UInt32) -> bool;

pub uninterp spec fn IsSupportedVoltageLevel(s: S, domain: UInt32, level: Int32) -> bool;

pub uninterp spec fn IsVoltageLevelSetRequestSupported(s: S, domain: UInt32, f: UInt64, level: Int32) -> bool;

pub uninterp spec fn AgentMaySetVoltageLevel(s: S, agent: UInt32, domain: UInt32) -> bool;

pub uninterp spec fn Bits64(value: UInt64, hi: int, lo: int) -> UInt64;

pub uninterp spec fn VoltageLevel(s: S, domain: UInt32) -> Int32;

pub uninterp spec fn VoltageLevelSetQueued(s: S, domain: UInt32, level: Int32) -> bool;

pub uninterp spec fn CompletesWithVoltageLevelSetCompleteMessage(s: S, domain: UInt32) -> bool;

} // verus!
