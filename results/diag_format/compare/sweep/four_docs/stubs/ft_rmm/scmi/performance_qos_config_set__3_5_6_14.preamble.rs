use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type Bits32 = u32;
pub type QosCapabilityDescriptor = u32;
pub type AgentId = u32;
pub type MessageId = u32;

pub struct S {
    pub qos_values: Map<(UInt32, QosCapabilityDescriptor), UInt32>,
    pub perf_domains: Set<UInt32>,
    pub queued: bool,
    pub delayed_sent: bool,
}

pub const SUCCESS: Int32 = 0;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;

pub const PERFORMANCE_QOS_CONFIG_COMPLETE: MessageId = 0x1Fu32;

pub const agent: AgentId = 0u32;
pub const result: Int32 = -100;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidPerfDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidQosCapability(s: S, domain_id: UInt32, capability: QosCapabilityDescriptor) -> bool;

pub open spec fn CountSetBits(x: u32) -> int;

pub open spec fn AreValidQosConfigFlags(s: S, flags: Bits32) -> bool;

pub open spec fn IsSupportedQosValue(s: S, domain_id: UInt32, capability: QosCapabilityDescriptor, qos_value: UInt32) -> bool;

pub open spec fn AgentMayConfigureQos(s: S, a: AgentId, domain_id: UInt32, capability: QosCapabilityDescriptor) -> bool;

pub open spec fn QosValue(s: S, domain_id: UInt32, capability: QosCapabilityDescriptor) -> UInt32;

pub open spec fn PlatformDefaultQosValue(s: S, domain_id: UInt32, capability: QosCapabilityDescriptor) -> UInt32;

pub open spec fn SiblingDomains(s: S, domain_id: UInt32) -> Set<UInt32>;

pub open spec fn AllPerfDomains(s: S) -> Set<UInt32>;

pub open spec fn QosConfigRequestQueued(s: S, domain_id: UInt32, capability: QosCapabilityDescriptor, flags: Bits32, qos_value: UInt32) -> bool;

pub open spec fn DelayedResponseSent(s: S, msg: MessageId) -> bool;

} // verus!
