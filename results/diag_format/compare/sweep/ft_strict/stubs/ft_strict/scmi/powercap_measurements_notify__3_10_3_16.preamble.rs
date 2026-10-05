use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0i32;
pub spec const NOT_FOUND: Int32 = -4i32;
pub spec const INVALID_PARAMETERS: Int32 = -2i32;
#[allow(non_upper_case_globals)]
pub spec const result: Int32 = -100i32;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;
pub open spec fn AreValidMeasurementsNotifyParameters(s: S, domain_id: UInt32, notify_enable: UInt32, power_thresh_low: UInt32, power_thresh_high: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn CallingAgent(s: S) -> AgentId;
pub open spec fn Bits(v: UInt32, hi: UInt32, lo: UInt32) -> UInt32;
pub open spec fn MeasurementsNotifyEnabled(s: S, agent: AgentId, domain_id: UInt32) -> UInt32;
pub open spec fn PowerThresholdLow(s: S, agent: AgentId, domain_id: UInt32) -> UInt32;
pub open spec fn PowerThresholdHigh(s: S, agent: AgentId, domain_id: UInt32) -> UInt32;
pub open spec fn AveragePowerOverMai(s: S, domain_id: UInt32) -> UInt32;
pub open spec fn MaiChanged(s: S, domain_id: UInt32) -> bool;
pub open spec fn SendsMeasurementsChanged(s: S, agent: AgentId, domain_id: UInt32) -> bool;

} // verus!
