use vstd::prelude::*;
verus! {

pub type HartId = u64;

pub struct SbiRet {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_DENIED: i64 = -4;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;

pub open spec fn CallingVirtualHart(s: S) -> HartId;

pub open spec fn StealTimeReportingEnabled(s: S, hart: HartId) -> bool;

pub open spec fn StealTimeShmemBase(s: S, hart: HartId) -> int;

pub open spec fn PhysMemByte(s: S, addr: int) -> u8;

} // verus!
