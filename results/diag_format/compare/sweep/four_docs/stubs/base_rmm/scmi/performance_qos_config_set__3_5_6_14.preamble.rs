use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub struct Req {
    pub domain_id: UInt32,
    pub flags: Seq<int>,
    pub capability: Seq<int>,
    pub qos_value: UInt32,
    pub agent: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;

pub const PERFORMANCE_QOS_CONFIG_COMPLETE: UInt32 = 0x10;

pub open spec fn req_value() -> Req;

pub spec const req: Req = req_value();

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidPerfDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidQosCapability(s: S, domain_id: UInt32, capability: Seq<int>) -> bool;

pub open spec fn CountSetBits(bits: Seq<int>) -> int;

pub open spec fn AreValidQosConfigFlags(s: S, flags: Seq<int>) -> bool;

pub open spec fn IsSupportedQosValue(s: S, domain_id: UInt32, capability: Seq<int>, qos_value: UInt32) -> bool;

pub open spec fn AgentMayConfigureQos(s: S, agent: UInt32, domain_id: UInt32, capability: Seq<int>) -> bool;

pub open spec fn QosValue(s: S, domain_id: UInt32, capability: Seq<int>) -> UInt32;

pub open spec fn PlatformDefaultQosValue(s: S, domain_id: UInt32, capability: Seq<int>) -> UInt32;

pub open spec fn SiblingDomains(s: S, domain_id: UInt32) -> Set<UInt32>;

pub open spec fn AllPerfDomains(s: S) -> Set<UInt32>;

pub open spec fn QosConfigRequestQueued(s: S, domain_id: UInt32, capability: Seq<int>, flags: Seq<int>, qos_value: UInt32) -> bool;

pub open spec fn DelayedResponseSent(s: S, message_id: UInt32) -> bool;

} // verus!
