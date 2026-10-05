use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;

pub type FunctionId = u32;

pub type HardwareConfiguration = u64;

pub struct S {
    pub powered_off: bool,
    pub cold_boot_next: bool,
    pub saved_config: HardwareConfiguration,
}

pub spec const NOT_SUPPORTED: PsciReturnCode = -1;

pub spec const INVALID_PARAMETERS: PsciReturnCode = -2;

pub spec const SYSTEM_OFF2: FunctionId = 0x84000015;

pub spec const BaseHardwareConfiguration: HardwareConfiguration = 0;

pub open spec fn IsFunctionImplemented(f: FunctionId) -> bool;

pub open spec fn AreInputParametersValid() -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn SystemIsPoweredOffFromCallerView(s: S) -> bool;

pub open spec fn NextStartIsColdBoot(s: S) -> bool;

pub open spec fn SavedToNonVolatileStorage(cfg: HardwareConfiguration, s: S) -> bool;

} // verus!
