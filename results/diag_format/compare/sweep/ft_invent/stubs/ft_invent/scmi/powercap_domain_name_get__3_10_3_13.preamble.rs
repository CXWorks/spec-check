use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type int32 = i32;

pub struct S {
    pub dummy: u32,
}

pub spec const SUCCESS: Result<int32, [u8; 64]> = Ok(0i32);

} // verus!
