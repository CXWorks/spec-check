use vstd::prelude::*;

verus! {

pub enum SdeiStatusCode {
    Success,
    NotSupported,
    InvalidParameters,
    Denied,
    Pending,
    OutOfResource,
}

pub spec const NOT_SUPPORTED: SdeiStatusCode = SdeiStatusCode::NotSupported;

pub struct S {
    pub sdei_supported: bool,
    pub vendor_version: int,
}

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(result: Result<int, SdeiStatusCode>, code: SdeiStatusCode) -> bool;

pub open spec fn VendorDefinedVersion() -> int;

pub open spec fn SdeiImplementsAllCalls() -> bool;

} // verus!
