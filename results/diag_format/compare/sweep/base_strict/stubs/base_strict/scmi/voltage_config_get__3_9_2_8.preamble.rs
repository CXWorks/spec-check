use vstd::prelude::*;
verus! {

// NOTE: the function calls IsRequestSupported with two arguments
// (message_id(old_s), protocol_id(old_s)) and also with one argument
// (message_id(old_s)). Verus spec fns have a fixed arity and cannot be
// overloaded, so no single declaration accepts both calls. This preamble
// declares the two-argument form. The one-argument call in the fourth
// conjunct will not type-check until the function is fixed.

pub type int32 = i32;
pub type uint32 = u32;

pub type DomainId = u32;
pub type MessageId = u32;
pub type ProtocolId = u32;
pub type AgentId = u32;

pub struct S {
    pub domain_id: DomainId,
    pub message_id: MessageId,
    pub protocol_id: ProtocolId,
    pub calling_agent: AgentId,
}

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_SUPPORTED: int32 = (-1) as int32;
pub spec const NOT_FOUND: int32 = (-4) as int32;
pub spec const DENIED: int32 = (-3) as int32;

pub open spec fn domain_id(s: S) -> DomainId;
pub open spec fn message_id(s: S) -> MessageId;
pub open spec fn protocol_id(s: S) -> ProtocolId;
pub open spec fn calling_agent(s: S) -> AgentId;

pub open spec fn IsValidVoltageDomain(d: DomainId) -> bool;
pub open spec fn IsRequestSupported(m: MessageId, p: ProtocolId) -> bool;
pub open spec fn AgentMayGetVoltageConfig(a: AgentId, d: DomainId) -> bool;
pub open spec fn ResultEqual(r: int32, code: int32) -> bool;
pub open spec fn Bits(x: uint32, hi: int, lo: int) -> uint32;
pub open spec fn VoltageDomainMode(d: DomainId) -> uint32;

} // verus!
