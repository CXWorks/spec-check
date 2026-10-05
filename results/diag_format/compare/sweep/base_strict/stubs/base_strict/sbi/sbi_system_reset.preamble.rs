use vstd::prelude::*;

verus! {

pub enum sbiret {
    SBI_SUCCESS,
    SBI_ERR_FAILED,
    SBI_ERR_NOT_SUPPORTED,
    SBI_ERR_INVALID_PARAM,
    SBI_ERR_DENIED,
    SBI_ERR_INVALID_ADDRESS,
    SBI_ERR_ALREADY_AVAILABLE,
}

pub struct S {
    pub reset_type: u32,
    pub reset_reason: u32,
}

pub open spec fn CallReturns() -> bool;

pub open spec fn SystemShutdown() -> bool;

pub open spec fn SystemColdReboot() -> bool;

pub open spec fn SystemWarmReboot() -> bool;

pub open spec fn SupervisorRunsNatively(s: S) -> bool;

pub open spec fn PhysicalPowerDownOfEntireSystem() -> bool;

pub open spec fn PhysicalPowerCycleOfEntireSystem() -> bool;

pub open spec fn PowerCycleOfMainProcessorAndPartsOfSystem() -> bool;

} // verus!
