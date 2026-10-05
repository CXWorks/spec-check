use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = 1;

pub open spec fn PowerDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn PowerDomainExtNameEqual(s: S, domain_id: UInt32, ext_name: Seq<u8>) -> bool;

} // verus!
