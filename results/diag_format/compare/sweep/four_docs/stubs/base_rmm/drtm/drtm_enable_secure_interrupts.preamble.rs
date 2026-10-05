use vstd::prelude::*;
verus! {

pub type Int64 = i64;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const DENIED: Int64 = -3;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;
pub open spec fn IsDrtmSupported(s: S) -> bool;
pub open spec fn HasDynamicLaunchOccurred(s: S) -> bool;
pub open spec fn AreSecureInterruptsDisabled(s: S) -> bool;
pub open spec fn DrtmParametersRequestedSecureInterruptDisable(s: S) -> bool;
pub open spec fn AreSecureInterruptsEnabled(s: S) -> bool;
pub open spec fn AreSecureInterruptsInUseByPlatform(s: S) -> bool;
pub open spec fn SecureInterruptEnableState(s: S) -> bool;

} // verus!
