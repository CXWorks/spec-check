use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub open spec fn DynamicLaunchRequestProxiedToCoprocessorDcrtm(s: S) -> bool;

pub open spec fn CallerDmaProtectionsBlockNonSecureDevices(s: S) -> bool;

pub open spec fn CallerDmaProtectionsBlockSecureDevices(s: S) -> bool;

} // verus!
