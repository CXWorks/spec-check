use vstd::prelude::*;
verus! {

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const NOT_FOUND: i32 = -2;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsValidPowercapDomain(s: S, domain_id: u32) -> bool;

pub open spec fn IsValidPowercapCpli(s: S, domain_id: u32, cpli: u32) -> bool;

pub open spec fn PowercapDomainSupportsCpc(s: S, domain_id: u32) -> bool;

pub open spec fn PowercapCaiGetSupported(s: S, domain_id: u32) -> bool;

pub open spec fn PowercapCaiOf(s: S, domain_id: u32, cpli: u32) -> u32;

} // verus!
