use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub enum Result<A, B> {
    Ok(A),
    Err(B),
}

impl<A, B> Result<A, B> {
    pub uninterp spec fn spec_index(self, i: int) -> UInt32;
}

pub struct S {
    pub num_protocols: UInt32,
}

pub spec const SUCCESS: Result<Int32, [UInt32; 1]> = Result::Ok(0i32);
pub spec const INVALID_PARAMETERS: Result<Int32, [UInt32; 1]> = Result::Ok(-2i32);

pub spec const BASE_PROTOCOL_ID: UInt32 = 0x10u32;

pub uninterp spec fn IsValidSkip(s: S, skip: UInt32) -> bool;

pub uninterp spec fn ResultEqual(a: Result<Int32, [UInt32; 1]>, b: Result<Int32, [UInt32; 1]>) -> bool;

pub uninterp spec fn ProtocolIdAt(arr: UInt32, i: UInt32) -> UInt32;

pub uninterp spec fn AccessibleProtocolAt(s: S, idx: int) -> UInt32;

pub uninterp spec fn ArrayElement(arr: UInt32, idx: UInt32) -> UInt32;

pub uninterp spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

} // verus!
