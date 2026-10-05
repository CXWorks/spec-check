use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type UInt64 = u64;

pub type RsiCommandReturnCode = u64;

pub spec const RSI_SUCCESS: RsiCommandReturnCode = 0;

pub struct S {
    pub dummy: u64,
}

pub open spec fn VendorImplementationVersion() -> UInt32;

pub open spec fn IsEarlierImplementationVersion(v: UInt32) -> bool;

} // verus!
