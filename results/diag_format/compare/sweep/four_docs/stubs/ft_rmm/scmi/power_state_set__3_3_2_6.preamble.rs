use vstd::prelude::*;
verus! {

pub struct UInt32 {
    pub r#async: u32,
    pub value: u32,
}

pub type Int32 = i32;

pub type AgentId = u32;

pub type ApId = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;

#[allow(non_upper_case_globals)]
pub const result: Int32 = 1;

#[allow(non_upper_case_globals)]
pub const caller: AgentId = 0;

#[allow(non_upper_case_globals)]
pub const caller_ap: ApId = 0;

pub open spec fn PowerDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

pub open spec fn IsValidPowerState(s: S, domain_id: UInt32, power_state: UInt32) -> bool;

pub open spec fn IsRequestSupported(s: S, domain_id: UInt32, flags: UInt32, power_state: UInt32) -> bool;

pub open spec fn AgentMaySetPowerDomainState(s: S, agent: AgentId, domain_id: UInt32) -> bool;

pub open spec fn PowerDomainStateMatches(s: S, domain_id: UInt32, power_state: UInt32) -> bool;

pub open spec fn IsSyncOnlyDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsApDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn PowerStateChangeScheduled(s: S, domain_id: UInt32, power_state: UInt32) -> bool;

pub open spec fn CommandReturnedBeforeApPowerDown(s: S, ap: ApId) -> bool;

pub open spec fn TransitionStartsOnWfiObserved(s: S, domain_id: UInt32, power_state: UInt32) -> bool;

} // verus!
