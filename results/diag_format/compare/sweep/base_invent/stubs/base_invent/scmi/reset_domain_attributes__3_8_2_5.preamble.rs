use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub type DomainId = u64;

pub struct S {
    pub domain_id: DomainId,
    pub reset_domains: Set<DomainId>,
}

} // verus!
