use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt8 = u8;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const FAILURE: Int32 = 1;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsNullTerminatedAsciiString(s: Seq<UInt8>, max_len: int) -> bool;

pub open spec fn IsSubVendorName(s: Seq<UInt8>) -> bool;

} // verus!
