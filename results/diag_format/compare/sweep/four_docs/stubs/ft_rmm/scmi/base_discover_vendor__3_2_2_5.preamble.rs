use vstd::prelude::*;
verus! {

pub type uint8 = u8;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsAsciiString(s: [uint8; 16]) -> bool;

} // verus!
