use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int64 = i64;
pub type UInt64 = u64;
pub type PeId = u64;

pub struct S {
    pub sdei_supported: bool,
    pub current_pe: PeId,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub open spec fn IsSdeiSupported(s: S) -> bool;

pub open spec fn IsValidContextParamId(param_id: UInt32) -> bool;

pub open spec fn CurrentPe(s: S) -> PeId;

pub open spec fn IsHandlerRunningOnPe(s: S, pe: PeId) -> bool;

pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;

pub open spec fn EventContextRegister(s: S, pe: PeId, param_id: UInt32) -> Int64;

} // verus!
