use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint8 = u8;

pub struct S {
    pub dummy: int,
}

pub open spec fn ResultIsSuccess(status: int32) -> bool;

pub open spec fn IsNullTerminatedAsciiString(s: [uint8; 16], max_len: int) -> bool;

} // verus!
