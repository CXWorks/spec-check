use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint8 = u8;

pub struct S {
    pub vendor_identifier: [uint8; 16],
    pub discovered: bool,
}

} // verus!
