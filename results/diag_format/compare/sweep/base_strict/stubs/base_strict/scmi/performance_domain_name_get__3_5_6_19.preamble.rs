use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub struct S {
    pub domain: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -6;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn PerformanceDomainExists(domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

pub open spec fn IsNullTerminatedAsciiString(s: [UInt8; 64], max_len: int) -> bool;

pub open spec fn PerformanceDomainExtendedName(domain_id: UInt32) -> [UInt8; 64];

} // verus!
