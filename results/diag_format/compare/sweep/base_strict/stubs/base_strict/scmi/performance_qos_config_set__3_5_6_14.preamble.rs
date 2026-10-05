use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type PerfDomain = u32;
pub type MessageId = u32;

pub struct S {
    pub domain_id: PerfDomain,
    pub capability: UInt32,
    pub flags: UInt32,
    pub qos_value: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;

pub const PERFORMANCE_QOS_CONFIG_COMPLETE: MessageId = 0x20;

pub open spec fn domain_id(s: S) -> PerfDomain;
pub open spec fn capability(s: S) -> UInt32;
pub open spec fn flags(s: S) -> UInt32;
pub open spec fn qos_value(s: S) -> UInt32;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn IsValidPerfDomain(d: PerfDomain) -> bool;
pub open spec fn IsValidQosCapability(d: PerfDomain, cap: UInt32) -> bool;
pub open spec fn PopCount(x: UInt32) -> nat;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn IsValidQosConfigFlags(f: UInt32) -> bool;
pub open spec fn IsSupportedQosValue(d: PerfDomain, cap: UInt32, v: UInt32) -> bool;
pub open spec fn CallerMayConfigureQos(d: PerfDomain, cap: UInt32) -> bool;
pub open spec fn QosValue(d: PerfDomain, cap: UInt32) -> UInt32;
pub open spec fn PlatformDefaultQos(d: PerfDomain, cap: UInt32) -> UInt32;
pub open spec fn IsSiblingDomain(a: PerfDomain, b: PerfDomain) -> bool;
pub open spec fn QosConfigRequestQueued(d: PerfDomain, cap: UInt32, f: UInt32, v: UInt32) -> bool;
pub open spec fn CompletesWithDelayedResponse(m: MessageId) -> bool;
pub open spec fn SendsDelayedResponse(m: MessageId) -> bool;

} // verus!
