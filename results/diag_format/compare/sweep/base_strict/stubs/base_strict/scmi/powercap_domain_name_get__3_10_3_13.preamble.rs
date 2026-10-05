use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub struct S {
    pub current_domain_id: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -6;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn PowercapDomainExists(id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 64], max_len: int) -> bool;

pub open spec fn IsPowercapDomainExtendedName(name: [UInt8; 64], id: UInt32) -> bool;

} // verus!
