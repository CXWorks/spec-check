use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = 1;

pub struct S {
    pub dummy: u64,
}

} // verus!
