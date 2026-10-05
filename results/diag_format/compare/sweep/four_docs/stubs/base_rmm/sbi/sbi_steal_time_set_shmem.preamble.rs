use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub cmd_input_shmem_phys_lo: u64,
    pub cmd_input_shmem_phys_hi: u64,
    pub cmd_input_flags: u64,
    pub calling_hart: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_DENIED: i64 = -4;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: i64 = -6;

pub const TRUE: bool = true;
pub const FALSE: bool = false;

pub open spec fn Concat(hi: u64, lo: u64) -> u64;

pub open spec fn IsAllOnes(x: u64) -> bool;

pub open spec fn StealTimeShmemBase(s: S, hart: u64) -> u64;

pub open spec fn StealTimeReportingEnabled(s: S, hart: u64) -> bool;

pub open spec fn Mem(s: S, addr: u64, len: int) -> Seq<u8>;

pub open spec fn IsZero(m: Seq<u8>) -> bool;

} // verus!
