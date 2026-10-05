use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct Agent {
    pub id: u32,
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1) as i32;
pub spec const NOT_FOUND: Int32 = (-4) as i32;
pub spec const DENIED: Int32 = (-3) as i32;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsPowercapMeasurementsGetSupported(s: S, domain_id: UInt32) -> bool;

pub open spec fn AgentMayGetPowercapMeasurements(s: S, agent: Agent, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn AveragePowerOverLatestInterval(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PowercapMeasurementAveragingInterval(s: S, domain_id: UInt32) -> UInt32;

} // verus!
