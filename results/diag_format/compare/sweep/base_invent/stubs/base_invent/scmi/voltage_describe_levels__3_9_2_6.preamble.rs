use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub const BE: int32 = 0;
pub const NOT_FOUND: int32 = -1;
pub const OUT_OF_RANGE: int32 = -2;
pub const NOT_SUPPORTED: int32 = -3;
pub const DENIED: int32 = -4;

} // verus!
