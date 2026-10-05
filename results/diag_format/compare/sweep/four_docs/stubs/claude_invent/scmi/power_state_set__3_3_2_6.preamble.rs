use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub domain_ids: Set<u32>,
    pub domain_states: Map<u32, u32>,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;
pub const DENIED: i32 = -3;
pub const NOT_FOUND: i32 = -4;

pub open spec fn PowerDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidPowerStateForDomain(s: S, domain_id: UInt32, power_state: UInt32) -> bool;

pub open spec fn AgentAllowedToSetPowerState(s: S, domain_id: UInt32) -> bool;

pub open spec fn PowerStateSetRequestSupported(s: S, domain_id: UInt32, flags: UInt32, power_state: UInt32) -> bool;

pub open spec fn IsApplicationProcessorDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn PowerStateTransitionScheduledOnWfi(s: S, domain_id: UInt32, power_state: UInt32) -> bool;

pub open spec fn PowerDomainState(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PowerStateTransitionScheduled(s: S, domain_id: UInt32, power_state: UInt32) -> bool;

} // verus!
