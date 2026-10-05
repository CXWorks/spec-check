use vstd::prelude::*;

verus! {

pub type UInt = u64;
pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub const SBI_PMU_CFG_FLAG_SKIP_MATCH: UInt = 1;
pub const SBI_PMU_CFG_FLAG_CLEAR_VALUE: UInt = 2;
pub const SBI_PMU_CFG_FLAG_AUTO_START: UInt = 4;

pub open spec fn FlagSet(flags: UInt, flag: UInt) -> bool;

pub open spec fn CounterStarted(s: S, counter: UInt) -> bool;

// NOTE: the function calls CounterCanMonitor with both 2 and 3 arguments.
// Rust/Verus has no function overloading, so only one arity can exist.
// This declares the 3-argument form (used at every call site but the first).
pub open spec fn CounterCanMonitor(s: S, counter: UInt, event_idx: UInt) -> bool;

pub open spec fn FirstCounterInSet(counter_idx_base: UInt, counter_idx_mask: UInt) -> UInt;

pub open spec fn CounterEvent(s: S, counter: UInt) -> UInt;

pub open spec fn CounterEventData(s: S, counter: UInt) -> UInt64;

pub open spec fn CounterValue(s: S, counter: UInt) -> u64;

// NOTE: the function calls CounterInSet with both 3 and 4 arguments.
// Only one arity can be declared; this is the 4-argument form (used at every call site but the first).
pub open spec fn CounterInSet(s: S, counter: UInt, counter_idx_base: UInt, counter_idx_mask: UInt) -> bool;

// NOTE: the function text is cut off partway through its last line
// (`CounterCanMonitor(old_s, selected_counter, event_idx` never closes).
// That unclosed delimiter is a syntax error in the function itself, and no
// declaration can fix it, so the preamble is unchanged.

} // verus!
