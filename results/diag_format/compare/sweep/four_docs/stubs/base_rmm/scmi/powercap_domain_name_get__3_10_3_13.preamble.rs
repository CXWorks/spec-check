use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = 1;

pub spec const domain_id: UInt32 = 0;

pub open spec fn PowercapDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn PowercapDomainExtendedName(domain_id: UInt32) -> [UInt8; 64];

} // verus!
