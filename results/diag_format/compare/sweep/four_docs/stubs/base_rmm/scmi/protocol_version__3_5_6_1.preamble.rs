use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub protocol_version: uint32,
    pub initialized: bool,
}

pub open spec fn StatusIndicatesSuccess(status: int32) -> bool;

} // verus!
