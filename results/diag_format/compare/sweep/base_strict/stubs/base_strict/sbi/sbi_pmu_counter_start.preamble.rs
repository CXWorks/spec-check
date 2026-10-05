use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type CounterIndex = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

impl sbiret {
    pub open spec fn is_ok(self) -> bool;

    pub open spec fn is_err(self) -> bool;
}

pub struct S {
    pub counter_idx_base: u64,
    pub counter_idx_mask: u64,
    pub start_flags: u64,
    pub initial_value: u64,
}

pub open spec fn InCounterSet(base: u64, mask: u64, i: CounterIndex) -> bool;

pub open spec fn CounterStarted(s: S, i: CounterIndex) -> bool;

pub open spec fn Bits(x: u64, hi: int, lo: int) -> u64;

pub open spec fn CounterValue(s: S, i: CounterIndex) -> u64;

pub open spec fn IsValidCounter(i: CounterIndex) -> bool;

pub open spec fn SnapshotShmemCounterValue(s: S, i: CounterIndex) -> u64;

pub open spec fn PreCounterValue(s: S, i: CounterIndex) -> u64;

} // verus!
