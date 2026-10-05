use vstd::prelude::*;
verus! {

pub type Int64 = i64;
pub type UInt64 = u64;
pub type PeId = u64;

pub struct S {
    pub sdei_enabled: bool,
    pub handler_running: Map<PeId, bool>,
    pub regs: Map<PeId, Seq<Int64>>,
}

pub spec const NOT_SUPPORTED: Int64 = (-1int) as i64;
pub spec const INVALID_PARAMETERS: Int64 = (-2int) as i64;
pub spec const DENIED: Int64 = (-3int) as i64;

#[allow(non_upper_case_globals)]
pub spec const param_id: UInt64 = 0;

pub open spec fn IsSdeiSupported() -> bool;
pub open spec fn IsValidContextParamId(id: UInt64) -> bool;
pub open spec fn CurrentPe() -> PeId;
pub open spec fn IsHandlerRunningOnPe(pe: PeId) -> bool;
pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;
pub open spec fn EventContextRegister(pe: PeId, id: UInt64) -> Int64;

} // verus!
