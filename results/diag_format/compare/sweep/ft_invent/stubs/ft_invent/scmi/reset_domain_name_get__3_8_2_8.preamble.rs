use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type int32 = i32;
pub type uint8 = u8;

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: Result<int32, ()> = Ok(0i32);
pub spec const NOT_FOUND: Result<int32, ()> = Ok(1i32);

} // verus!
