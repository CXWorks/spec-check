use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type SdeiStatusCode = i64;
pub type PeIndex = u64;

pub struct S {
    pub sdei_supported: bool,
    pub dummy: u64,
}

pub spec const NOT_SUPPORTED: SdeiStatusCode = (-1) as i64;
pub spec const INVALID_PARAMETERS: SdeiStatusCode = (-2) as i64;
pub spec const DENIED: SdeiStatusCode = (-3) as i64;

pub spec const SDEI_SUCCESS: Result<(), SdeiStatusCode> = Ok(());

pub uninterp spec fn IsSdeiSupported(s: S) -> bool;
pub uninterp spec fn IsValidContextParamId(s: S, param_id: UInt32) -> bool;
pub uninterp spec fn IsHandlerRunningOnPe(s: S, pe: PeIndex) -> bool;
pub uninterp spec fn CurrentPe() -> PeIndex;
pub uninterp spec fn ResultEqual(result: Result<(), SdeiStatusCode>, code: SdeiStatusCode) -> bool;
pub uninterp spec fn EventContextRegister(s: S, pe: PeIndex, param_id: UInt32) -> Result<(), SdeiStatusCode>;

} // verus!
