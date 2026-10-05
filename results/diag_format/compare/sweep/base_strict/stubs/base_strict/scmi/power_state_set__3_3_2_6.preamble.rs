use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type PowerDomainId = u32;
pub type PowerStateValue = u32;
pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as i32;
pub spec const DENIED: Int32 = (-3int) as i32;
pub spec const NOT_FOUND: Int32 = (-4int) as i32;

pub spec const domain_id: PowerDomainId = 1;
pub spec const power_state: PowerStateValue = 2;
pub spec const calling_agent: AgentId = 3;
pub spec const flags: UInt64 = 0;

pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub uninterp spec fn PowerDomainExists(s: S, d: PowerDomainId) -> bool;
pub uninterp spec fn IsValidPowerState(s: S, d: PowerDomainId, ps: PowerStateValue) -> bool;
pub uninterp spec fn IsRequestSupported(s: S, f: UInt64, d: PowerDomainId, ps: PowerStateValue) -> bool;
pub uninterp spec fn AgentMaySetPowerState(s: S, a: AgentId, d: PowerDomainId) -> bool;
pub uninterp spec fn IsSynchronousRequest(s: S, f: UInt64, d: PowerDomainId) -> bool;
pub uninterp spec fn IsApplicationProcessorDomain(s: S, d: PowerDomainId) -> bool;
pub uninterp spec fn PowerDomainInState(s: S, d: PowerDomainId, ps: PowerStateValue) -> bool;
pub uninterp spec fn Bits64(v: UInt64, hi: int, lo: int) -> UInt64;
pub uninterp spec fn PowerStateChangeScheduled(s: S, d: PowerDomainId, ps: PowerStateValue) -> bool;
pub uninterp spec fn ReturnsBeforeApPowerDown(s: S, d: PowerDomainId) -> bool;
pub uninterp spec fn TransitionBeginsOnWfi(s: S, d: PowerDomainId, ps: PowerStateValue) -> bool;
pub uninterp spec fn IsParentDomain(s: S, p: PowerDomainId, d: PowerDomainId) -> bool;
pub uninterp spec fn PowerStateRequestedForParent(s: S, p: PowerDomainId, ps: PowerStateValue) -> bool;

} // verus!
