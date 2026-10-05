use vstd::prelude::*;
verus! {

pub type PsciReturnCode = i32;
pub type FunctionId = u32;
pub type CoreId = u64;
pub type PowerState = u8;

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const DENIED: PsciReturnCode = -3;

pub const CPU_FREEZE: FunctionId = 0x8400000B;

pub const IMPLEMENTATION_DEFINED_LOW_POWER_STATE: PowerState = 3;

pub struct Core {
    pub power_state: PowerState,
}

pub struct S {
    pub cores: Seq<Core>,
    pub implemented: Set<FunctionId>,
}

pub open spec fn IsImplemented(s: S, fid: FunctionId) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn CpuOffWouldBeDenied(s: S, core: CoreId) -> bool;

pub open spec fn CurrentCore() -> CoreId;

pub open spec fn CoreAt(s: S, core: CoreId) -> Core;

pub open spec fn WakeupInterruptsResumeExecution(s: S, core: CoreId) -> bool;

pub open spec fn InterruptsRemainPendingOrActive(s: S, core: CoreId) -> bool;

} // verus!
