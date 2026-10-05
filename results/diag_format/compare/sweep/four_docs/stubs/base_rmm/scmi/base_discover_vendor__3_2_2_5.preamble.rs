use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint8 = u8;

pub struct S {
    pub state_placeholder: int,
}

pub open spec fn IsAsciiString(s: [uint8; 16]) -> bool;

} // verus!
