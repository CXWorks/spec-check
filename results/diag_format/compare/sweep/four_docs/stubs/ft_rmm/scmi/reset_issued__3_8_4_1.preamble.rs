use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsRegisteredForResetNotification(s: S, agent_id: UInt32, domain_id: UInt32) -> bool;

pub open spec fn ResetDomainHasBeenReset(s: S, domain_id: UInt32, reset_state: UInt32) -> bool;

} // verus!
