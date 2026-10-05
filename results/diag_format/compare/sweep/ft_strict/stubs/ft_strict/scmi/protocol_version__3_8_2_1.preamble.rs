use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsSuccessStatus(status: int32) -> bool;

pub open spec fn ProtocolVersionIs(version: uint32, major: uint32, minor: uint32) -> bool;

} // verus!
