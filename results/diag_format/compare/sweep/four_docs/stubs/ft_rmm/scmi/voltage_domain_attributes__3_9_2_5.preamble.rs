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
    pub name: [UInt8; 16],
}

pub type RsiCommandReturnCode = Int32;

pub struct Result<T, E> {
    pub status: Int32,
    pub ok: bool,
    pub value: Option<T>,
    pub error: Option<E>,
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool {
        self.ok
    }

    pub open spec fn is_Err(self) -> bool {
        !self.ok
    }
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = -4;

pub spec const RSI_SUCCESS: Result<VoltageDomainAttributes, RsiCommandReturnCode> = Result {
    status: 0,
    ok: true,
    value: Option::None,
    error: Option::None,
};

pub open spec fn VoltageDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn VoltageDomainSupportsAsyncLevelSet(s: S, domain_id: UInt32) -> bool;

pub open spec fn VoltageDomainNameLength(s: S, domain_id: UInt32) -> int;

pub open spec fn VoltageDomainName(s: S, domain_id: UInt32) -> Seq<UInt8>;

pub open spec fn IsNullTerminatedAscii(s: S, name: [UInt8; 16], len: int) -> bool;

pub open spec fn LowerBytes(s: S, bytes: Seq<UInt8>, n: int) -> [UInt8; 16];

pub open spec fn ResultEqual<A, B>(a: A, b: B) -> bool;

} // verus!
