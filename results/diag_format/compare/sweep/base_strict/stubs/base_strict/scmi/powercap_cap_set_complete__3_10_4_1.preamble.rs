use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PowerCapRequestCompleted(domain_id: UInt32, power_cap: UInt32) -> bool;

pub open spec fn DomainSupportsCpc(domain_id: UInt32) -> bool;

} // verus!
