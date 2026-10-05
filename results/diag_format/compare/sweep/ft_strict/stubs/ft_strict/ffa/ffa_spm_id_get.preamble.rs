use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type FfaInstance = u8;

pub type ExceptionLevel = u8;

pub enum RsiCommandReturnCode {
    Success,
    ErrorInput,
    ErrorState,
    Incomplete,
}

pub struct S {
    pub dummy: u64,
}

pub const RSI_SUCCESS: UInt32 = 0;

pub const NOT_SUPPORTED: UInt32 = 0xFFFF_FFFF;

pub const FFA_SPM_ID_GET: UInt32 = 0x8400_0085;

pub const NON_SECURE_PHYSICAL: FfaInstance = 0;

pub const NON_SECURE_VIRTUAL: FfaInstance = 1;

pub const SECURE_PHYSICAL: FfaInstance = 2;

pub const SECURE_VIRTUAL: FfaInstance = 3;

pub const S_EL1: ExceptionLevel = 1;

pub const S_EL2: ExceptionLevel = 2;

pub const EL3: ExceptionLevel = 3;

pub open spec fn IsImplementedAtInstance(s: S, function_id: UInt32, instance: FfaInstance) -> bool;

pub open spec fn CallerInstance(s: S) -> FfaInstance;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn SpmcId(s: S) -> UInt32;

pub open spec fn SpmdId(s: S) -> UInt32;

pub open spec fn SpmcExceptionLevel(s: S) -> ExceptionLevel;

} // verus!
