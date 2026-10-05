use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i64;
pub type PsciFunctionId = u32;

pub const RSI_SUCCESS: PsciReturnCode = 0;
pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;

pub const SYSTEM_OFF2: PsciFunctionId = 0x84000015;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsFunctionImplemented(s: S, fid: PsciFunctionId) -> bool;

pub open spec fn AreInputParametersValid(s: S) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn SystemPoweredOffFromCallerView(s: S) -> bool;

pub open spec fn NextStartIsColdBoot(s: S) -> bool;

pub open spec fn BaseHardwareConfigSavedToNonVolatileStorage(s: S) -> bool;

pub open spec fn MemoryLayoutPreservedForNextBoot(s: S) -> bool;

pub open spec fn StaticDevicePropertiesPreservedForNextBoot(s: S) -> bool;

pub open spec fn OsVisibleFirmwareInterfacesPreservedForNextBoot(s: S) -> bool;

} // verus!
