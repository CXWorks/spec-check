use vstd::prelude::*;
verus! {

pub type UInt8 = u8;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsAsciiString(s: [UInt8; 16]) -> bool;

} // verus!
