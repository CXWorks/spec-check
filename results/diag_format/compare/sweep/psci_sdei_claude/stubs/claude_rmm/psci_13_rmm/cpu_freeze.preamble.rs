use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = i32;

pub type CoreId = u64;

pub type PowerState = u8;

pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const DENIED: PsciReturnCode = -3;

pub const CPU_FREEZE: UInt64 = 0x8400000B;

pub const ON_POWER_STATE: PowerState = 0;
pub const OFF_POWER_STATE: PowerState = 1;
pub const IMPLEMENTATION_DEFINED_LOW_POWER_STATE: PowerState = 2;

pub struct Core {
    pub power_state: PowerState,
}

pub struct S {
    pub cores: Seq<Core>,
    pub current_core: CoreId,
}

pub open spec fn IsImplemented(fid: UInt64) -> bool;

pub open spec fn ResultEqual(result: PsciReturnCode, expected: PsciReturnCode) -> bool;

pub open spec fn CpuOffWouldBeDenied(s: S, core: CoreId) -> bool;

pub open spec fn CurrentCore(s: S) -> CoreId;

pub open spec fn CoreAt(s: S, core: CoreId) -> Core;

} // verus!
