use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: u32,
}

pub open spec fn VendorImplementationVersion() -> uint32;

pub open spec fn IsEarlierImplementationVersion(v: UInt32) -> bool;

} // verus!
