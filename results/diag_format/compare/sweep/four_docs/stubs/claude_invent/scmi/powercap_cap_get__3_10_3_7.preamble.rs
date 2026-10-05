use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = 1;
pub const NOT_SUPPORTED: i32 = 2;

pub open spec fn PowercapDomainIsValid(s: S, domain_id: u32) -> bool;
pub open spec fn PowercapCpliIsValid(s: S, domain_id: u32, cpli: u32) -> bool;
pub open spec fn PowercapCapGetIsSupported(s: S, domain_id: u32) -> bool;
pub open spec fn PowercapEnforcedCap(s: S, domain_id: u32, cpli: u32) -> u32;

} // verus!
