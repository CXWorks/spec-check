use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub protocol_version: u32,
    pub initialized: bool,
}

} // verus!
