use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub implementation_version: u32,
}

pub open spec fn VendorImplementationVersion() -> uint32;

pub open spec fn VendorImplementationVersionOfAnyOlderImplementation(s: S) -> uint32;

} // verus!
