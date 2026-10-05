use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;
pub type DomainId = u32;

pub struct S {
    pub current_domain: DomainId,
    pub domain_count: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -6;

pub open spec fn domain_id(s: S) -> DomainId;

pub open spec fn ResetDomainExists(id: DomainId) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn ResetDomainExtendedName(id: DomainId) -> [UInt8; 64];

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 64], len: int) -> bool;

} // verus!
