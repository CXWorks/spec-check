use vstd::prelude::*;

verus! {

pub type UInt2 = u8;
pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct Result<A, B, C>(pub A, pub C, pub B);

impl<A, B, C> Result<A, B, C> {
    pub uninterp spec fn is_Ok(self) -> bool;
}

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -6;

pub uninterp spec fn PinctrlEntityExists(s: S, selector: UInt2, identifier: UInt16) -> bool;

pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub uninterp spec fn PinctrlExtendedName(s: S, selector: UInt2, identifier: UInt16) -> [UInt8; 64];

pub uninterp spec fn IsNullTerminatedAscii(name: [UInt8; 64], len: int) -> bool;

} // verus!
