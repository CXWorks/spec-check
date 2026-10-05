use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type UInt32 = u32;

pub const SUCCESS: int32 = 0;
pub const INVALID_PARAMETERS: int32 = 1;

pub struct S {
    pub notify_enable: UInt32,
}

} // verus!
