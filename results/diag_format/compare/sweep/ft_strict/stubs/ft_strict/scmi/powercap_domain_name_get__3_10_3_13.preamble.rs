use vstd::prelude::*;

verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = (-4int) as i32;
pub spec const result: Int32 = 1;

pub uninterp spec fn PowercapDomainExists(s: S, domain_id: UInt32) -> bool;

pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub uninterp spec fn IsNullTerminatedAscii(name: [UInt8; 64], len: int) -> bool;

pub uninterp spec fn IsPowercapDomainExtendedName(name: [UInt8; 64], domain_id: UInt32) -> bool;

} // verus!
