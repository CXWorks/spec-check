use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub enum SbiReturnCode {
    Success,
    Failed,
    NotSupported,
    InvalidParam,
    Denied,
    InvalidAddress,
    AlreadyAvailable,
}

#[allow(non_camel_case_types)]
pub struct SBI_SUCCESS;

pub open spec fn ResultEqual(a: SbiReturnCode, b: SBI_SUCCESS) -> bool;

pub struct S {
    pub dummy: int,
}

pub open spec fn NumHardwareCounters() -> UInt;

pub open spec fn NumFirmwareCounters() -> UInt;

} // verus!
