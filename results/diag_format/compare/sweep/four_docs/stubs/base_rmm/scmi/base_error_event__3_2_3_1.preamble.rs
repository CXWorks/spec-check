use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub error_status: u32,
    pub fatal_error: bool,
    pub initial_boot: bool,
}

} // verus!
