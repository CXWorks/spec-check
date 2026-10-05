use vstd::prelude::*;

verus! {

pub type PsciReturnCode = u64;

pub type FunctionId = u64;

pub type HardwareConfiguration = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const RSI_SUCCESS: PsciReturnCode = 0;

pub spec const NOT_SUPPORTED: PsciReturnCode = 1;

pub spec const INVALID_PARAMETERS: PsciReturnCode = 2;

pub spec const SYSTEM_OFF2: FunctionId = 0x84000015;

pub spec const BaseHardwareConfiguration: HardwareConfiguration = 7;

pub open spec fn IsFunctionImplemented(s: S, fid: FunctionId) -> bool;

pub open spec fn ResultEqual(result: PsciReturnCode, expected: PsciReturnCode) -> bool;

pub open spec fn AreInputParametersValid(s: S) -> bool;

pub open spec fn SystemIsPoweredOffFromCallerView(s: S) -> bool;

pub open spec fn NextStartIsColdBoot(s: S) -> bool;

pub open spec fn SavedToNonVolatileStorage(s: S, cfg: HardwareConfiguration) -> bool;

} // verus!
