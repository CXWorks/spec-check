use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type FunctionId = u32;

pub enum RsiCommandReturnCode {
    NotSupported,
    InvalidParameters,
    Denied,
    Other,
}

pub struct S {
    pub dummy: int,
}

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Result::Ok(());
pub spec const NOT_SUPPORTED: Result<(), RsiCommandReturnCode> = Result::Err(RsiCommandReturnCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), RsiCommandReturnCode> = Result::Err(RsiCommandReturnCode::InvalidParameters);

pub spec const SYSTEM_RESET2: FunctionId = 0x84000012u32;
pub spec const SYSTEM_WARM_RESET: UInt32 = 0u32;

pub open spec fn IsImplemented(s: S, fid: FunctionId) -> bool;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn ResetAppliesToCallerMachineView(s: S, reset_type: UInt32) -> bool;
pub open spec fn PreservedMainMemoryUnchanged(s: S) -> bool;
pub open spec fn MemoryRequestersReset(s: S) -> bool;
pub open spec fn AllCpusAndMmusReset(s: S) -> bool;
pub open spec fn AllInterruptsDisabled(s: S) -> bool;
pub open spec fn SmmusInColdResetState(s: S) -> bool;
pub open spec fn CookieIgnored(s: S, cookie: UInt64) -> bool;
pub open spec fn VendorResetPerformed(s: S, reset_type: UInt32, cookie: UInt64) -> bool;

} // verus!
