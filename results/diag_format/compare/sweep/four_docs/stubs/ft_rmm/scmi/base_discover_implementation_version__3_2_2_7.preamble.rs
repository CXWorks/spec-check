use vstd::prelude::*;
verus! {

pub type uint32 = u32;

pub struct S {
    pub vendor_implementation_version: uint32,
}

pub open spec fn VendorImplementationVersion(s: S) -> uint32;

} // verus!
