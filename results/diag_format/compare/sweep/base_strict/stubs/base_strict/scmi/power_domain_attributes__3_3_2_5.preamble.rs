use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub struct S {
    pub domain_id: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PowerDomainExists(domain_id: UInt32) -> bool;

pub open spec fn PowerStateChangeNotifySupported(domain_id: UInt32) -> bool;

pub open spec fn PowerStateAsyncSetSupported(domain_id: UInt32) -> bool;

pub open spec fn PowerStateSyncSetSupported(domain_id: UInt32) -> bool;

pub open spec fn PowerStateChangeRequestedNotifySupported(domain_id: UInt32) -> bool;

pub open spec fn PowerDomainNameLength(domain_id: UInt32) -> int;

pub open spec fn PowerDomainName(domain_id: UInt32) -> [UInt8; 16];

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 16], len: int) -> bool;

pub open spec fn NullTerminatedPrefix(name: [UInt8; 16], len: int) -> [UInt8; 16];

} // verus!
