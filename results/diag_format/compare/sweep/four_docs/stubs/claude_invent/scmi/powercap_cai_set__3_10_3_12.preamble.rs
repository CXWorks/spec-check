use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;
pub const DENIED: i32 = -3;
pub const NOT_FOUND: i32 = -4;

pub open spec fn IsPowercapDomainValid(s: S, domain_id: u32) -> bool;
pub open spec fn IsPowercapCpliValid(s: S, domain_id: u32, cpli: u32) -> bool;
pub open spec fn PowercapDomainSupportsCpc(s: S, domain_id: u32) -> bool;
pub open spec fn PowercapDomainSupportsCaiConfig(s: S, domain_id: u32) -> bool;
pub open spec fn IsPowercapCaiSupported(s: S, domain_id: u32, cai: u32) -> bool;
pub open spec fn IsAgentAllowedToSetPowercapCai(s: S, domain_id: u32) -> bool;
pub open spec fn PowercapCai(s: S, domain_id: u32, cpli: u32) -> u32;
pub open spec fn PowercapStateUnchangedExceptCai(old_s: S, new_s: S, domain_id: u32, cpli: u32) -> bool;

} // verus!
