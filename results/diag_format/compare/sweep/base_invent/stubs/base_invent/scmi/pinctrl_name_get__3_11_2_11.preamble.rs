use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type uint8 = u8;

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -2;

pub struct S {
    pub dummy: int,
}

} // verus!
