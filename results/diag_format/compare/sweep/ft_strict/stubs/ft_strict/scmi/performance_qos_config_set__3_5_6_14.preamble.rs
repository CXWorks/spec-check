use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type PerfDomain = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = -4;
pub spec const INVALID_PARAMETERS: Int32 = -2;
pub spec const DENIED: Int32 = -3;
pub spec const result: Int32 = -100;

pub spec const PERFORMANCE_QOS_CONFIG_COMPLETE: UInt32 = 0x10;

pub open spec fn IsValidPerfDomain(s: S, domain_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn IsValidQosCapability(s: S, domain_id: UInt32, capability: UInt32) -> bool;
pub open spec fn PopCount(x: UInt32) -> int;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn IsValidQosConfigFlags(s: S, flags: UInt32) -> bool;
pub open spec fn IsSupportedQosValue(s: S, domain_id: UInt32, capability: UInt32, qos_value: UInt32) -> bool;
pub open spec fn CallerMayConfigureQos(s: S, domain_id: UInt32, capability: UInt32) -> bool;
pub open spec fn QosValue(s: S, domain_id: PerfDomain, capability: UInt32) -> UInt32;
pub open spec fn PlatformDefaultQos(s: S, domain_id: PerfDomain, capability: UInt32) -> UInt32;
pub open spec fn IsSiblingDomain(s: S, d: PerfDomain, domain_id: PerfDomain) -> bool;
pub open spec fn QosConfigRequestQueued(s: S, domain_id: UInt32, capability: UInt32, flags: UInt32, qos_value: UInt32) -> bool;
pub open spec fn CompletesWithDelayedResponse(s: S, msg: UInt32) -> bool;
pub open spec fn SendsDelayedResponse(s: S, msg: UInt32) -> bool;

} // verus!
