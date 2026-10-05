use vstd::prelude::*;

verus! {

pub type ScmiStatus = i32;

pub const SUCCESS: ScmiStatus = 0;
pub const NOT_FOUND: ScmiStatus = -4;
pub const INVALID_PARAMETERS: ScmiStatus = -2;

pub struct S {
    pub dummy: int,
}

pub open spec fn PowerDomainIsValid(s: S, domain_id: u32) -> bool;

pub open spec fn PowerStateNotifyImplemented(s: S) -> bool;

pub open spec fn PowerStateNotifyEnabled(s: S, agent_id: u32, domain_id: u32) -> bool;

pub open spec fn UnchangedExceptPowerStateNotify(old_s: S, new_s: S, agent_id: u32, domain_id: u32) -> bool;

} // verus!
