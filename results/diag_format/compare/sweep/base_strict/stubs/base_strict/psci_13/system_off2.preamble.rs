use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;

pub type PsciFunctionId = u32;

pub type SysPowerState = u32;

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: PsciReturnCode = -1;

pub const INVALID_PARAMETERS: PsciReturnCode = -2;

pub const SYSTEM_OFF2: PsciFunctionId = 0x84000015;

pub const SYS_POWER_STATE_OFF: SysPowerState = 0;

pub open spec fn IsFunctionImplemented(fid: PsciFunctionId) -> bool;

pub open spec fn ResultEqual(result: PsciReturnCode, expected: PsciReturnCode) -> bool;

pub open spec fn AreInputParametersValid() -> bool;

pub open spec fn SystemPoweredOffFromCallerView() -> bool;

pub open spec fn NextStartIsColdBoot() -> bool;

pub open spec fn BaseHardwareConfigSavedToNonVolatileStorage() -> bool;

pub open spec fn MemoryLayoutPreservedForNextBoot() -> bool;

pub open spec fn StaticDevicePropertiesPreservedForNextBoot() -> bool;

pub open spec fn OsVisibleFirmwareInterfacesPreservedForNextBoot() -> bool;

pub open spec fn SystemPowerState() -> SysPowerState;

pub open spec fn NonVolatileSavedState() -> bool;

} // verus!
