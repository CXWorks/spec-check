use vstd::prelude::*;
verus! {

pub type Int64 = i64;

pub type UInt64 = u64;

pub type PeId = u64;

pub struct S {
    pub handler_running: Map<PeId, bool>,
    pub regs: Map<PeId, Seq<UInt64>>,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

#[allow(non_upper_case_globals)]
pub const param_id: UInt64 = 0;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn HandlerRunning(pe: PeId) -> bool;

pub open spec fn CallingPe() -> PeId;

pub open spec fn EventContextRegister(pe: PeId, param: UInt64) -> Int64;

} // verus!
