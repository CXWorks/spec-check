use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type SdeiStatusCode = i64;

pub type PeIndex = u64;

pub const NOT_SUPPORTED: SdeiStatusCode = -1;

pub const DENIED: SdeiStatusCode = -3;

pub struct S {
    pub sdei_supported: bool,
    pub calling_pe: PeIndex,
}

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn CallingPe(s: S) -> PeIndex;

pub open spec fn HandlerRunning(s: S, pe: PeIndex) -> bool;

pub open spec fn ResultEqual(result: Result<(), SdeiStatusCode>, code: SdeiStatusCode) -> bool;

} // verus!
