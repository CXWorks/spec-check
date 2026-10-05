use vstd::prelude::*;

verus! {

pub enum DrtmStatusCode {
    NotSupported,
    Denied,
    InvalidParameters,
}

pub struct S {
    pub drtm_supported: bool,
    pub dynamic_launch_occurred: bool,
    pub secure_interrupts_disabled: bool,
    pub secure_interrupts_enabled: bool,
    pub params_requested_secure_interrupt_disable: bool,
    pub secure_interrupts_in_use_by_platform: bool,
}

pub spec const SUCCESS: Result<(), DrtmStatusCode> = Ok(());

pub spec const NOT_SUPPORTED: Result<(), DrtmStatusCode> = Err(DrtmStatusCode::NotSupported);

pub spec const DENIED: Result<(), DrtmStatusCode> = Err(DrtmStatusCode::Denied);

pub open spec fn IsDrtmSupported(s: S) -> bool;

pub open spec fn HasDynamicLaunchOccurred(s: S) -> bool;

pub open spec fn AreSecureInterruptsDisabled(s: S) -> bool;

pub open spec fn DrtmParametersRequestedSecureInterruptDisable(s: S) -> bool;

pub open spec fn AreSecureInterruptsInUseByPlatform(s: S) -> bool;

pub open spec fn AreSecureInterruptsEnabled(s: S) -> bool;

pub open spec fn ResultEqual(a: Result<(), DrtmStatusCode>, b: Result<(), DrtmStatusCode>) -> bool;

} // verus!
