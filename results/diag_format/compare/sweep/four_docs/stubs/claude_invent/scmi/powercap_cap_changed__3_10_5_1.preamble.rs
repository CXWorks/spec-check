use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub open spec fn PowercapDomainIsValid(s: S, domain_id: u32) -> bool;

pub open spec fn PowercapAgentRegisteredForCapChangeNotification(s: S, domain_id: u32) -> bool;

pub open spec fn PowercapDomainCurrentPowerCap(s: S, domain_id: u32) -> u32;

pub open spec fn PowercapDomainCurrentCai(s: S, domain_id: u32) -> u32;

pub open spec fn PowercapDomainSupportsCpc(s: S, domain_id: u32) -> bool;

} // verus!
