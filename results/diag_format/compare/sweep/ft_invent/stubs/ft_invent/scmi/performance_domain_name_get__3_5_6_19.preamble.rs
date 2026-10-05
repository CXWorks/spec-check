use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

#[allow(non_camel_case_types)]
pub type int32 = i32;

pub struct S {
    pub dummy: u8,
}

pub spec const SUCCESS: Result<int32, [u8; 64]> = Ok(0i32);

} // verus!
