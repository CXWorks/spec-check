use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: int,
}

pub open spec fn ResetDomainExists(s: S, domain_id: u32) -> bool;

pub open spec fn ResetStateSupported(s: S, domain_id: u32, reset_state: u32) -> bool;

pub open spec fn AgentMayResetDomain(s: S, agent_id: u32, domain_id: u32) -> bool;

pub open spec fn OtherResetDomainsUnchanged(old_s: S, new_s: S, domain_id: u32) -> bool;

pub open spec fn ResetCompletePending(s: S, agent_id: u32, domain_id: u32) -> bool;

pub open spec fn DomainResetPerformed(old_s: S, new_s: S, domain_id: u32, reset_state: u32) -> bool;

pub open spec fn ResetSignalAsserted(s: S, domain_id: u32) -> bool;

} // verus!
