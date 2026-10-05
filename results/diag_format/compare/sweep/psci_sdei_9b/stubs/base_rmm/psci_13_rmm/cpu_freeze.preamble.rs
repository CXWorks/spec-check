use vstd::prelude::*;
verus! {

pub type PsciReturnCode = i32;
pub type PsciFunctionId = u32;
pub type CoreId = u64;

pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const DENIED: PsciReturnCode = -3;

pub const CPU_FREEZE: PsciFunctionId = 0x8400000B;

pub struct S {
    pub current_core: CoreId,
}

pub open spec fn IsImplemented(fid: PsciFunctionId) -> bool;

pub open spec fn ResultEqual(result: PsciReturnCode, expected: PsciReturnCode) -> bool;

pub open spec fn CurrentCore() -> CoreId;

pub open spec fn CpuOffWouldBeDenied(core: CoreId) -> bool;

} // verus!
