use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub enum FfaReturnCode {
    Success,
    Error(int),
}

pub type FfaErrorCode = int;

pub spec const NOT_SUPPORTED: FfaErrorCode = 1int;

pub uninterp spec fn FfaInterfaceOrFeatureIsImplemented(s: S, function_or_feature_id: UInt32, input_properties: UInt32) -> bool;

pub uninterp spec fn IsFfaError(result: FfaReturnCode, code: FfaErrorCode) -> bool;

pub uninterp spec fn IsFfaSuccess(result: FfaReturnCode) -> bool;

} // verus!
