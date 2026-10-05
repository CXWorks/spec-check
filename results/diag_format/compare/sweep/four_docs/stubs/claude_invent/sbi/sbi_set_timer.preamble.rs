use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct SbiRet {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub stimecmp: u64,
    pub time: u64,
    pub sip_stip: bool,
    pub sie_stie: bool,
}

pub const SBI_SUCCESS: i64 = 0;

pub open spec fn TimerCompareValue(s: S) -> UInt64;

pub open spec fn IsFutureTime(s: S, t: UInt64) -> bool;

pub open spec fn SupervisorTimerInterruptPending(s: S) -> bool;

pub open spec fn SieStie(s: S) -> bool;

} // verus!
