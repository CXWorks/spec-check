use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct NotSupported {
    pub code: u64,
}

pub struct S {
    pub drtm_supported: bool,
    pub version: u64,
}

pub spec const NOT_SUPPORTED: NotSupported = NotSupported { code: 0xFFFF_FFFF_FFFF_FFFFu64 };

pub open spec fn DrtmIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(result: Result<(), NotSupported>, code: NotSupported) -> bool;

} // verus!
