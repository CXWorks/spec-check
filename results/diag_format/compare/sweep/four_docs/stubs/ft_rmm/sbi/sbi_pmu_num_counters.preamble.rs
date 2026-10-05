use vstd::prelude::*;
verus! {

pub type UInt = u64;

pub type SbiReturnCode = i64;

pub const SBI_SUCCESS: SbiReturnCode = 0;
pub const SBI_ERR_FAILED: SbiReturnCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiReturnCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiReturnCode = -3;

pub struct S {
    pub dummy: u64,
}

pub open spec fn ResultEqual(a: SbiReturnCode, b: SbiReturnCode) -> bool;

pub open spec fn NumHardwareCounters() -> UInt;

pub open spec fn NumFirmwareCounters() -> UInt;

} // verus!
