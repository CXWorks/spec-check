use vstd::prelude::*;

verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;

pub const RSI_SUCCESS: Int32 = 1;
pub const result: Int32 = 2;

pub open spec fn PerformanceDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

pub open spec fn IsNullTerminatedAsciiString(name: [UInt8; 64], len: int) -> bool;

pub open spec fn PerformanceDomainExtendedName(s: S, domain_id: UInt32) -> [UInt8; 64];

} // verus!
