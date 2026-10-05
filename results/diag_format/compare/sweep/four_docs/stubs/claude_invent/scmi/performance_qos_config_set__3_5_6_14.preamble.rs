use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;
pub const DENIED: i32 = -3;

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: u32) -> bool;

pub open spec fn IsValidQosCapability(s: S, domain_id: u32, capability: u32) -> bool;

pub open spec fn IsPlatformQosCapability(s: S, capability: u32) -> bool;

pub open spec fn IsQosConfigFlagsSupported(s: S, domain_id: u32, capability: u32, flags: u32) -> bool;

pub open spec fn IsQosValueSupported(s: S, domain_id: u32, capability: u32, qos_value: u32) -> bool;

pub open spec fn IsAgentPermittedQosConfig(s: S, domain_id: u32, capability: u32) -> bool;

pub open spec fn QosValue(s: S, domain_id: u32, capability: u32) -> u32;

pub open spec fn QosPlatformDefault(s: S, domain_id: u32, capability: u32) -> u32;

pub open spec fn IsSiblingDomain(s: S, domain_id: u32, other: u32) -> bool;

} // verus!
