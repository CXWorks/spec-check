use vstd::prelude::*;
verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct PowerDomainAttributesReturn {
    pub attributes: UInt32,
    pub name: [UInt8; 16],
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Result<Int32, PowerDomainAttributesReturn> = Ok(0i32);
pub spec const NOT_FOUND: Result<Int32, PowerDomainAttributesReturn> = Ok(-4i32);

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn PowerDomainExists(s: S, domain: UInt32) -> bool;

pub open spec fn ResultEqual(a: Result<Int32, PowerDomainAttributesReturn>, b: Result<Int32, PowerDomainAttributesReturn>) -> bool;

pub open spec fn PowerStateChangeNotifySupported(s: S, domain: UInt32) -> bool;

pub open spec fn PowerStateAsyncSetSupported(s: S, domain: UInt32) -> bool;

pub open spec fn PowerStateSyncSetSupported(s: S, domain: UInt32) -> bool;

pub open spec fn PowerStateChangeRequestedNotifySupported(s: S, domain: UInt32) -> bool;

pub open spec fn PowerDomainNameLength(s: S, domain: UInt32) -> nat;

pub open spec fn PowerDomainName(s: S, domain: UInt32) -> [UInt8; 16];

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 16], len: int) -> bool;

pub open spec fn NullTerminatedPrefix(name: [UInt8; 16], len: int) -> [UInt8; 16];

} // verus!
