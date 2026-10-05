use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct AxisNameDesc {
    pub axis_id: uint32,
    pub name: Seq<u8>,
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: int32 = 0i32;
pub spec const NOT_SUPPORTED: int32 = -1i32;
pub spec const NOT_FOUND: int32 = -3i32;

} // verus!
