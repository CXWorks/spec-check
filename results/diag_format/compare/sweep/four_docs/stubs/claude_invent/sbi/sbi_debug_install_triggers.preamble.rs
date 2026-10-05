use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: u64,
}

pub open spec fn Xlen() -> int;

pub open spec fn ShmemWordLe(s: S, off: int) -> u64;

pub open spec fn TrigIdxHwTrigger(s: S, trig_idx: u64) -> u64;

pub open spec fn TrigIdxIsUnused(s: S, trig_idx: u64) -> bool;

pub open spec fn HwTriggerIsUnused(s: S, hw: u64) -> bool;

pub open spec fn HwTriggerMatchesConfig(s: S, hw: u64, tdata1: u64, tdata2: u64, tdata3: u64) -> bool;

pub open spec fn TrigStateSavedModeBits(s: S, trig_idx: u64) -> u64;

pub open spec fn Tdata1ModeBits(tdata1: u64) -> u64;

pub open spec fn HwTriggerTdata1(s: S, hw: u64) -> u64;

pub open spec fn HwTriggerTdata2(s: S, hw: u64) -> u64;

pub open spec fn HwTriggerTdata3(s: S, hw: u64) -> u64;

pub open spec fn TrigConfigChainedToNext(s: S, i: int) -> bool;

} // verus!
