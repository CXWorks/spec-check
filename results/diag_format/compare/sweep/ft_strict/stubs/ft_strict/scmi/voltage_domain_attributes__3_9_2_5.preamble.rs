use vstd::prelude::*;
verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub struct VoltageDomainAttributes {
    pub attributes: UInt32,
}

pub enum RsiCommandReturnCode {
    Success,
    NotFound,
    InvalidParameters,
    Denied,
}

pub spec const RSI_SUCCESS: Result<VoltageDomainAttributes, RsiCommandReturnCode> = Result::Ok(VoltageDomainAttributes { attributes: 0 });

pub spec const NOT_FOUND: RsiCommandReturnCode = RsiCommandReturnCode::NotFound;

pub spec const SUCCESS: Int32 = 0;

pub open spec fn ResultEqual<A, B>(a: A, b: B) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> int;

pub open spec fn Bit(x: UInt32, pos: int) -> int;

pub open spec fn VoltageDomainExists(s: S, domain_id: int) -> bool;

pub open spec fn VoltageDomainSupportsAsyncLevelSet(s: S, domain_id: int) -> bool;

pub open spec fn VoltageDomainNameLength(s: S, domain_id: int) -> int;

pub open spec fn VoltageDomainName(s: S, domain_id: int) -> Seq<UInt8>;

pub open spec fn NameEquals(name: [UInt8; 16], expected: Seq<UInt8>) -> bool;

pub open spec fn NameEqualsLowerBytes(name: [UInt8; 16], expected: Seq<UInt8>, n: int) -> bool;

} // verus!
