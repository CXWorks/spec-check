use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type PowerDomainId = u32;
pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;
pub const result: Int32 = -100;

pub const calling_agent: AgentId = 1;
pub const p: PowerDomainId = 2;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn PowerDomainExists(s: S, domain_id: UInt32) -> bool;
pub open spec fn IsValidPowerState(s: S, domain_id: UInt32, power_state: UInt32) -> bool;
pub open spec fn IsRequestSupported(s: S, flags: UInt32, domain_id: UInt32, power_state: UInt32) -> bool;
pub open spec fn AgentMaySetPowerState(s: S, agent: AgentId, domain_id: UInt32) -> bool;
pub open spec fn IsSynchronousRequest(s: S, flags: UInt32, domain_id: UInt32) -> bool;
pub open spec fn IsApplicationProcessorDomain(s: S, domain_id: UInt32) -> bool;
pub open spec fn PowerDomainInState(s: S, domain_id: UInt32, power_state: UInt32) -> bool;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn PowerStateChangeScheduled(s: S, domain_id: UInt32, power_state: UInt32) -> bool;
pub open spec fn ReturnsBeforeApPowerDown(s: S, domain_id: UInt32) -> bool;
pub open spec fn TransitionBeginsOnWfi(s: S, domain_id: UInt32, power_state: UInt32) -> bool;
pub open spec fn IsParentDomain(s: S, parent: PowerDomainId, domain_id: UInt32) -> bool;
pub open spec fn PowerStateRequestedForParent(s: S, parent: PowerDomainId, power_state: UInt32) -> bool;

} // verus!
