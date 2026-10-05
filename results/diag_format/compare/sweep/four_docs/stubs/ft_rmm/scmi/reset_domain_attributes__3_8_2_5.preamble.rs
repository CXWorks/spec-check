use vstd::prelude::*;
verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool;
    pub open spec fn is_Err(self) -> bool;
}

pub enum RmiStatusCode {
    Success,
    NotFound,
    InvalidParameters,
    Denied,
    NotSupported,
    GenericError,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub struct ResetDomainInfo {
    pub supports_async_reset: bool,
    pub supports_reset_notifications: bool,
    pub name_longer_than_16_bytes: bool,
    pub max_reset_latency_us: UInt32,
    pub name: [UInt8; 16],
}

pub struct S {
    pub num_reset_domains: UInt32,
}

pub open spec fn ResetDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResetDomain(s: S, domain_id: UInt32) -> ResetDomainInfo;

pub open spec fn ResultEqual<A, B>(a: A, b: B) -> bool;

pub open spec fn LowerBytes(name: [UInt8; 16], n: nat) -> [UInt8; 16];

} // verus!
