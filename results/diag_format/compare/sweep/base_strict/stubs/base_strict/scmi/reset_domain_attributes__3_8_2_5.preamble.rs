use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub type UInt8 = u8;

pub struct S {
    pub reset_domain_id: UInt32,
}

pub const SUCCESS: Int32 = 0;

pub const NOT_FOUND: Int32 = -4;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn IsValidResetDomain(domain: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(value: UInt32, high: int, low: int) -> UInt32;

pub open spec fn ResetDomainSupportsAsyncReset(domain: UInt32) -> bool;

pub open spec fn ResetDomainSupportsResetNotifications(domain: UInt32) -> bool;

pub open spec fn ResetDomainNameLength(domain: UInt32) -> nat;

pub open spec fn ResetLatencySupported(domain: UInt32) -> bool;

pub open spec fn MaxResetLatencyUs(domain: UInt32) -> UInt32;

pub open spec fn IsNullTerminatedAsciiString(s: [UInt8; 16], max_len: int) -> bool;

pub open spec fn ResetDomainName(domain: UInt32) -> [UInt8; 16];

pub open spec fn NullTerminatedPrefix(s: [UInt8; 16], len: int) -> [UInt8; 16];

} // verus!
