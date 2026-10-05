use vstd::prelude::*;

verus! {

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool {
        match self {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }

    pub open spec fn is_Err(self) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Err(_) => true,
        }
    }
}

pub type FfaErrorCode = i32;

pub const SUCCESS: FfaErrorCode = 0;
pub const NOT_SUPPORTED: FfaErrorCode = -1;
pub const INVALID_PARAMETERS: FfaErrorCode = -2;
pub const NO_MEMORY: FfaErrorCode = -3;
pub const BUSY: FfaErrorCode = -4;
pub const INTERRUPTED: FfaErrorCode = -5;
pub const DENIED: FfaErrorCode = -6;
pub const RETRY: FfaErrorCode = -7;
pub const ABORTED: FfaErrorCode = -8;

pub type UInt64 = u64;

pub struct S {
    pub security_state: u64,
    pub caller_el: u64,
    pub scr_el3: u64,
    pub is_secure_physical_smc: bool,
}

pub open spec fn FfaInstanceIsSecurePhysicalSmc(s: S) -> bool;

pub open spec fn CallerIsSEl1OrSEl2(s: S) -> bool;

pub open spec fn ScrEl3Fiq(s: S) -> u64;

pub open spec fn ResultEqual(result: Result<(), FfaErrorCode>, code: FfaErrorCode) -> bool;

pub open spec fn SecurityState(s: S) -> u64;

pub open spec fn CallerExceptionLevel(s: S) -> u64;

} // verus!
