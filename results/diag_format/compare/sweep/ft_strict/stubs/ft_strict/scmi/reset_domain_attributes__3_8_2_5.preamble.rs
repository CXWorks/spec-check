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
    pub open spec fn is_Ok(&self) -> bool {
        self is Ok
    }

    pub open spec fn is_Err(&self) -> bool {
        self is Err
    }
}

pub type RmiStatusCode = u32;

pub const SUCCESS: RmiStatusCode = 0;
pub const NOT_FOUND: RmiStatusCode = 1;

pub struct S {
    pub dummy: int,
}

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn IsValidResetDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> int;

pub open spec fn ResetDomainSupportsAsyncReset(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResetDomainSupportsResetNotifications(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResetDomainNameLength(s: S, domain_id: UInt32) -> int;

pub open spec fn ResetLatencySupported(s: S, domain_id: UInt32) -> bool;

pub open spec fn MaxResetLatencyUs(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn IsNullTerminatedAsciiString(name: [UInt8; 16], len: int) -> bool;

pub open spec fn ResetDomainName(s: S, domain_id: UInt32) -> [UInt8; 16];

pub open spec fn NullTerminatedPrefix(name: [UInt8; 16], len: int) -> [UInt8; 16];

} // verus!
