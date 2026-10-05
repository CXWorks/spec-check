use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub struct S {
    pub num_protocols_implemented: UInt32,
    pub protocol_list: Seq<UInt32>,
}

pub const SUCCESS: Int32 = 0;

pub const INVALID_PARAMETERS: Int32 = -2;

pub const BASE_PROTOCOL_ID: UInt32 = 16;

#[allow(non_upper_case_globals)]
pub const skip: UInt32 = 3;

pub open spec fn IsValidSkip(s: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn ProtocolIdAt<T>(protocols: UInt32, i: T) -> UInt32;

pub open spec fn AccessibleProtocolAt(idx: int) -> UInt32;

pub open spec fn ArrayElement<T>(arr: UInt32, idx: T) -> UInt32;

pub open spec fn Bits(value: UInt32, high: int, low: int) -> UInt32;

} // verus!
