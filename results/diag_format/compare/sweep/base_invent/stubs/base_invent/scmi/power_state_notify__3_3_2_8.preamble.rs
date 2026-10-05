use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub struct S {
    pub domain_id: int,
    pub num_domains: int,
    pub notify_enable: u32,
}

impl S {
    pub open spec fn power_state_notify_enabled(self, domain: int) -> int;
}

} // verus!
