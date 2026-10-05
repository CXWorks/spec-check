use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const NOT_FOUND: Int32 = -4;
pub const DENIED: Int32 = -3;

pub const calling_agent: AgentId = 7;
pub const result: Int32 = -100;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsPowercapMeasurementsGetSupported(s: S, domain_id: UInt32) -> bool;

pub open spec fn AgentMayGetPowercapMeasurements(s: S, agent: AgentId, domain_id: UInt32) -> bool;

pub open spec fn AveragePowerOverLatestInterval(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PowercapMeasurementAveragingInterval(s: S, domain_id: UInt32) -> UInt32;

} // verus!
