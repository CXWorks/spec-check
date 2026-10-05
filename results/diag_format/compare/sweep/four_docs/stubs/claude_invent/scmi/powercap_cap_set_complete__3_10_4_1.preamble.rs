use vstd::prelude::*;
verus! {

pub struct S {
    pub placeholder: int,
}

pub open spec fn PowercapDomainSupportsCpc(s: S, domain_id: u32) -> bool;

pub open spec fn PowercapDomainPowerCap(s: S, domain_id: u32, cpli: u32) -> u32;

} // verus!
