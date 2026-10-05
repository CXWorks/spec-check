use vstd::prelude::*;

verus! {

pub struct S {
    pub drtm_supported: bool,
    pub dynamic_launch_occurred: bool,
    pub secure_interrupts_disabled: bool,
}

pub const SUCCESS: i64 = 0;
pub const NOT_SUPPORTED: i64 = -1;
pub const DENIED: i64 = -3;

pub open spec fn DrtmIsSupported(s: S) -> bool;

pub open spec fn DynamicLaunchOccurred(s: S) -> bool;

pub open spec fn SecureInterruptsDisabled(s: S) -> bool;

} // verus!
