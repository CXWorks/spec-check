use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type Int64 = i64;

pub struct SbiRet {
    pub error: Int64,
    pub value: UInt64,
}

pub struct S {
    pub shmem_base: UInt64,
    pub num_triggers: UInt64,
}

pub const SBI_SUCCESS: Int64 = 0;
pub const SBI_ERR_FAILED: Int64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: Int64 = -2;
pub const SBI_ERR_INVALID_PARAM: Int64 = -3;

pub open spec fn ShmemTrigIdx(s: S, i: int) -> int;

pub open spec fn ShmemTrigTdata1(s: S, i: int) -> UInt64;

pub open spec fn IsInstalledDebugTrigger(s: S, idx: int) -> bool;

pub open spec fn InstalledTriggerTdata1(s: S, idx: int) -> UInt64;

pub open spec fn Tdata1Type(tdata1: UInt64) -> UInt64;

pub open spec fn Tdata1Chain(tdata1: UInt64) -> bool;

} // verus!
