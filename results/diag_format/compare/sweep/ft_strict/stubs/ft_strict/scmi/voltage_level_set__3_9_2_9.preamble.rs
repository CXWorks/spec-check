use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as i32;
pub spec const DENIED: Int32 = (-3int) as i32;
pub spec const NOT_FOUND: Int32 = (-4int) as i32;

#[allow(non_upper_case_globals)]
pub spec const calling_agent: UInt32 = 0;

#[allow(non_upper_case_globals)]
pub spec const result: Int32 = 1000;

pub uninterp spec fn VoltageDomainExists(s: S, domain_id: UInt32) -> bool;

pub uninterp spec fn IsSupportedVoltageLevel(s: S, domain_id: UInt32, voltage_level: Int32) -> bool;

pub uninterp spec fn IsVoltageLevelSetRequestSupported(s: S, domain_id: UInt32, flags: UInt32, voltage_level: Int32) -> bool;

pub uninterp spec fn AgentMaySetVoltageLevel(s: S, agent_id: UInt32, domain_id: UInt32) -> bool;

pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub uninterp spec fn Bits(value: UInt32, hi: int, lo: int) -> int;

pub uninterp spec fn VoltageLevel(s: S, domain_id: UInt32) -> Int32;

pub uninterp spec fn VoltageLevelSetQueued(s: S, domain_id: UInt32, voltage_level: Int32) -> bool;

pub uninterp spec fn CompletesWithVoltageLevelSetCompleteMessage(s: S, domain_id: UInt32) -> bool;

} // verus!
