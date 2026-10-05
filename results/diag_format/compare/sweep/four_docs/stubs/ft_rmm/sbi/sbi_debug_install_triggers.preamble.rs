use vstd::prelude::*;

verus! {

pub type unsigned_long = u64;

pub spec const XLEN: int = 64;

pub type TrigIdx = int;

pub struct TriggerConfig {
    pub trig_tdata1: u64,
    pub trig_tdata2: u64,
    pub trig_tdata3: u64,
}

pub struct HwTrigger {
    pub tdata1: u64,
    pub tdata2: u64,
    pub tdata3: u64,
}

pub type TriggerChain = Seq<int>;

pub struct S {
    pub hart_id: u64,
}

pub open spec fn TriggerConfigsProcessedInIncreasingIndexOrder(s: S, start: int, count: u64) -> bool;

pub open spec fn IsMappedUnusedHwTrigger(s: S, idx: int) -> bool;

pub open spec fn HwTriggerMatchesConfig(s: S, hw: HwTrigger, cfg: TriggerConfig) -> bool;

pub open spec fn TrigIdxAt(s: S, i: int) -> int;

pub open spec fn HwTriggerOf(s: S, idx: int) -> HwTrigger;

pub open spec fn TriggerConfigAt(s: S, i: int) -> TriggerConfig;

pub open spec fn TrigState(s: S, idx: int) -> u64;

pub open spec fn SavedPrivBits(s: S, tdata1: u64) -> u64;

pub open spec fn SharedMemWord(s: S, offset: int, word: int) -> int;

pub open spec fn TriggerConfigChains(s: S, start: int, count: u64) -> Set<TriggerChain>;

pub open spec fn TrigIdxValuesContiguous(s: S, chain: TriggerChain) -> bool;

pub open spec fn HwTriggersContiguous(s: S, chain: TriggerChain) -> bool;

} // verus!
