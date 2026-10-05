use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub const domain_id: UInt32 = 0;

pub struct ResetDomainInfo {
    pub supports_async_reset: bool,
    pub supports_reset_notifications: bool,
    pub name_longer_than_16_bytes: bool,
    pub max_reset_latency_us: UInt32,
    pub name: Seq<UInt8>,
}

pub struct S {
    pub reset_domains: Map<UInt32, ResetDomainInfo>,
}

pub open spec fn ResetDomainExists(s: S, id: UInt32) -> bool;

pub open spec fn ResetDomain(s: S, id: UInt32) -> ResetDomainInfo;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn LowerBytes(name: Seq<UInt8>, n: int) -> Seq<UInt8>;

} // verus!
