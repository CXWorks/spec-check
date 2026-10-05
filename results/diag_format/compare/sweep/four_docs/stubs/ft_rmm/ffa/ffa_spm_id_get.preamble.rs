use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;

pub enum Instance {
    NonSecurePhysical,
    NonSecureVirtual,
    SecurePhysical,
    SecureVirtual,
}

pub enum ExceptionLevel {
    EL0,
    EL1,
    EL2,
    EL3,
    S_EL0,
    S_EL1,
    S_EL2,
}

pub enum FFAReturn {
    Success,
}

pub enum FFAStatusCode {
    NotSupported,
    InvalidParameters,
    Denied,
}

pub struct S {
    pub dummy: int,
}

pub spec const NON_SECURE_PHYSICAL: Instance = Instance::NonSecurePhysical;
pub spec const NON_SECURE_VIRTUAL: Instance = Instance::NonSecureVirtual;
pub spec const SECURE_PHYSICAL: Instance = Instance::SecurePhysical;
pub spec const SECURE_VIRTUAL: Instance = Instance::SecureVirtual;

pub spec const S_EL1: ExceptionLevel = ExceptionLevel::S_EL1;
pub spec const S_EL2: ExceptionLevel = ExceptionLevel::S_EL2;
pub spec const EL3: ExceptionLevel = ExceptionLevel::EL3;

pub spec const FFA_SPM_ID_GET: UInt32 = 0x84000085u32;

pub spec const FFA_SUCCESS: Result<FFAReturn, FFAStatusCode> = Ok(FFAReturn::Success);
pub spec const NOT_SUPPORTED: Result<FFAReturn, FFAStatusCode> = Err(FFAStatusCode::NotSupported);

pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32, instance: Instance) -> bool;

pub open spec fn ResultEqual(r1: Result<FFAReturn, FFAStatusCode>, r2: Result<FFAReturn, FFAStatusCode>) -> bool;

pub open spec fn SpmcId() -> UInt16;

pub open spec fn SpmdId() -> UInt16;

pub open spec fn SpmcEl() -> ExceptionLevel;

} // verus!
