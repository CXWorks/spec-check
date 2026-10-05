use vstd::prelude::*;

verus! {

pub type UInt32 = Seq<u32>;

pub type Int32 = i32;

pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;

pub spec const INVALID_PARAMETERS: Int32 = (-2int) as i32;

pub spec const NOT_FOUND: Int32 = (-4int) as i32;

pub spec const result: Int32 = 1;

pub spec const caller: AgentId = 7;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn AreValidParameters(s: S, domain_id: UInt32, notify_enable: UInt32, power_thresh_low: UInt32, power_thresh_high: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn MeasurementsNotifyEnabled(s: S, agent: AgentId, domain_id: UInt32) -> bool;

pub open spec fn PowerThreshLow(s: S, agent: AgentId, domain_id: UInt32) -> UInt32;

pub open spec fn PowerThreshHigh(s: S, agent: AgentId, domain_id: UInt32) -> UInt32;

} // verus!
