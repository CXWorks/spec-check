use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsSuccessStatus(status: Int32) -> bool;

} // verus!
