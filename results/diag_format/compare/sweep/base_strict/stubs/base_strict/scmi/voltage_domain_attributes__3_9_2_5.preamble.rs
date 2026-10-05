use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub struct S {
    pub domain_id: UInt32,
}

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn Bit(x: UInt32, pos: int) -> UInt32;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn VoltageDomainExists(domain_id: UInt32) -> bool;

pub open spec fn VoltageDomainSupportsAsyncLevelSet(domain_id: UInt32) -> bool;

pub open spec fn VoltageDomainNameLength(domain_id: UInt32) -> int;

pub open spec fn VoltageDomainName(domain_id: UInt32) -> Seq<UInt8>;

pub open spec fn NameEquals(name: [UInt8; 16], expected: Seq<UInt8>) -> bool;

pub open spec fn NameEqualsLowerBytes(name: [UInt8; 16], expected: Seq<UInt8>, n: int) -> bool;

} // verus!
