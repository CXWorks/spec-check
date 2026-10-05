use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub struct PowerStateSetFlags {
    pub r#async: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;

pub spec const domain_id: UInt32 = 0;
pub spec const power_state: UInt32 = 0;
pub spec const flags: PowerStateSetFlags = PowerStateSetFlags { r#async: 0 };
pub spec const caller: UInt32 = 0;
pub spec const caller_ap: UInt32 = 1;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn PowerDomainExists(s: S, domain_id: UInt32) -> bool;
pub open spec fn IsValidPowerState(s: S, domain_id: UInt32, power_state: UInt32) -> bool;
pub open spec fn IsRequestSupported(s: S, domain_id: UInt32, flags: PowerStateSetFlags, power_state: UInt32) -> bool;
pub open spec fn AgentMaySetPowerDomainState(s: S, caller: UInt32, domain_id: UInt32) -> bool;
pub open spec fn IsSyncOnlyDomain(s: S, domain_id: UInt32) -> bool;
pub open spec fn IsApDomain(s: S, domain_id: UInt32) -> bool;
pub open spec fn PowerDomainStateMatches(s: S, domain_id: UInt32, power_state: UInt32) -> bool;
pub open spec fn PowerStateChangeScheduled(s: S, domain_id: UInt32, power_state: UInt32) -> bool;
pub open spec fn CommandReturnedBeforeApPowerDown(s: S, caller_ap: UInt32) -> bool;
pub open spec fn TransitionStartsOnWfiObserved(s: S, domain_id: UInt32, power_state: UInt32) -> bool;

} // verus!
