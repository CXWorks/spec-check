use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const DENIED: Int64 = -3;

pub open spec fn IsDrtmSupported() -> bool;

pub open spec fn DynamicLaunchOccurred() -> bool;

pub open spec fn SecureInterruptsDisabled() -> bool;

pub open spec fn SecureInterruptDisableRequestedInDrtmParameters() -> bool;

pub open spec fn SecureInterruptsEnabled() -> bool;

pub open spec fn SecureInterruptsInUseByPlatform() -> bool;

pub open spec fn SecureInterruptEnableState() -> u64;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

} // verus!
