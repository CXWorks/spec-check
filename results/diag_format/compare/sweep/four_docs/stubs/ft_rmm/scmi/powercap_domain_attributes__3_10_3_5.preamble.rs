use vstd::prelude::*;

verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;

pub struct S {
    pub placeholder: int,
}

pub enum RmiStatusCode {
    Success,
    NotFound,
    InvalidParameters,
    Denied,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = -2;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn PowercapDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn MaiIsConfigurable(s: S, domain_id: UInt32) -> bool;

pub open spec fn PowerCapIsConfigurable(s: S, domain_id: UInt32) -> bool;

pub open spec fn CaiIsConfigurable(s: S, domain_id: UInt32) -> bool;

} // verus!
