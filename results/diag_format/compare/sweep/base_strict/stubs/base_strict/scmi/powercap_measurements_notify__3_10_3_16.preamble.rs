use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NOT_FOUND: Int32 = -4;

pub const domain_id: UInt32 = 1;
pub const notify_enable: UInt32 = 2;
pub const power_thresh_low: UInt32 = 3;
pub const power_thresh_high: UInt32 = 4;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidPowercapDomain(s: S, did: UInt32) -> bool;

pub open spec fn AreValidMeasurementsNotifyParameters(s: S, did: UInt32, ne: UInt32, ptl: UInt32, pth: UInt32) -> bool;

pub open spec fn CallingAgent() -> AgentId;

pub open spec fn MeasurementsNotifyEnabled(s: S, agent: AgentId, did: UInt32) -> UInt32;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn PowerThresholdLow(s: S, agent: AgentId, did: UInt32) -> UInt32;

pub open spec fn PowerThresholdHigh(s: S, agent: AgentId, did: UInt32) -> UInt32;

} // verus!
