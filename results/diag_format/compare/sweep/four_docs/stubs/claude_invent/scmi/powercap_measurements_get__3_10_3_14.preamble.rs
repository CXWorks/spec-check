use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const NOT_FOUND: i32 = -4;
pub const DENIED: i32 = -3;

pub open spec fn PowercapDomainIsValid(s: S, domain_id: u32) -> bool;

pub open spec fn PowercapMeasurementsSupported(s: S, domain_id: u32) -> bool;

pub open spec fn PowercapAgentAllowedMeasurements(s: S, domain_id: u32) -> bool;

pub open spec fn PowercapAveragePower(s: S, domain_id: u32) -> u32;

pub open spec fn PowercapMai(s: S, domain_id: u32) -> u32;

} // verus!
