use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub struct S {
    pub dummy: int,
}

pub spec const domain_id: u32 = 0u32;

pub const SUCCESS: Int32 = 0i32;
pub const NOT_FOUND: Int32 = -4i32;

pub open spec fn VoltageDomainExists(s: S, id: int) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn VoltageDomainSupportsAsyncLevelSet(s: S, id: int) -> bool;

pub open spec fn VoltageDomainNameLength(s: S, id: int) -> u32;

pub open spec fn IsNullTerminatedAscii(name: Seq<UInt8>, len: int) -> bool;

pub open spec fn VoltageDomainName(s: S, id: int) -> Seq<UInt8>;

pub open spec fn LowerBytes(bytes: Seq<UInt8>, n: int) -> Seq<UInt8>;

} // verus!
