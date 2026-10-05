use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type Int64 = i64;

pub struct SbiRet {
    pub error: Int64,
    pub value: Int64,
}

pub struct S {
    pub num_harts: UInt64,
    pub hart_states: Map<UInt64, UInt64>,
    pub hart_start_addrs: Map<UInt64, UInt64>,
    pub hart_start_opaques: Map<UInt64, UInt64>,
    pub hart_start_modes: Map<UInt64, UInt64>,
}

pub const SBI_SUCCESS: Int64 = 0;
pub const SBI_ERR_FAILED: Int64 = -1;
pub const SBI_ERR_INVALID_PARAM: Int64 = -3;
pub const SBI_ERR_INVALID_ADDRESS: Int64 = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: Int64 = -6;

pub const SBI_HSM_STATE_STARTED: UInt64 = 0;
pub const SBI_HSM_STATE_STOPPED: UInt64 = 1;
pub const SBI_HSM_STATE_START_PENDING: UInt64 = 2;

pub const PRIV_MODE_SUPERVISOR: UInt64 = 1;

pub open spec fn IsValidHartId(s: S, hartid: UInt64) -> bool;

pub open spec fn IsValidStartAddr(s: S, start_addr: UInt64) -> bool;

pub open spec fn HartStateOf(s: S, hartid: UInt64) -> UInt64;

pub open spec fn HartStartAddr(s: S, hartid: UInt64) -> UInt64;

pub open spec fn HartStartOpaque(s: S, hartid: UInt64) -> UInt64;

pub open spec fn HartStartMode(s: S, hartid: UInt64) -> UInt64;

} // verus!
