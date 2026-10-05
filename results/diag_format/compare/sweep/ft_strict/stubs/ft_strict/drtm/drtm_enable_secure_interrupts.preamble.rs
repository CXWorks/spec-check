use vstd::prelude::*;
verus! {

pub enum DrtmCommandReturnCode {
    NotSupported,
    Denied,
    InvalidParameters,
}

pub struct S {
    pub drtm_supported: bool,
    pub dynamic_launch_occurred: bool,
    pub secure_interrupts_disabled: bool,
    pub secure_interrupts_enabled: bool,
    pub secure_interrupt_disable_requested: bool,
    pub secure_interrupts_in_use_by_platform: bool,
}

pub spec const SUCCESS: Result<(), DrtmCommandReturnCode> = Ok(());

pub spec const NOT_SUPPORTED: Result<(), DrtmCommandReturnCode> = Err(DrtmCommandReturnCode::NotSupported);

pub spec const DENIED: Result<(), DrtmCommandReturnCode> = Err(DrtmCommandReturnCode::Denied);

pub open spec fn ResultEqual(a: Result<(), DrtmCommandReturnCode>, b: Result<(), DrtmCommandReturnCode>) -> bool;

pub open spec fn IsDrtmSupported(s: S) -> bool;

pub open spec fn DynamicLaunchOccurred(s: S) -> bool;

pub open spec fn SecureInterruptsDisabled(s: S) -> bool;

pub open spec fn SecureInterruptDisableRequestedInDrtmParameters(s: S) -> bool;

pub open spec fn SecureInterruptsEnabled(s: S) -> bool;

pub open spec fn SecureInterruptsInUseByPlatform(s: S) -> bool;

} // verus!
