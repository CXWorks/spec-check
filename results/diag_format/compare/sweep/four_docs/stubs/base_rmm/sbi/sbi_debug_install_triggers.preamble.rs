use vstd::prelude::*;

verus! {

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub shmem_base: u64,
    pub num_triggers: u64,
}

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

pub type TrigStateVal = u64;

pub type TriggerChain = Seq<int>;

pub spec const XLEN: int = 64;

pub open spec fn TriggerConfigsProcessedInIncreasingIndexOrder(start: int, end: int) -> bool;

pub open spec fn trig_count(s: S) -> int;

pub open spec fn IsMappedUnusedHwTrigger(idx: int) -> bool;

pub open spec fn TrigIdxAt(i: int, s: S) -> int;

pub open spec fn HwTriggerMatchesConfig(hw: HwTrigger, cfg: TriggerConfig) -> bool;

pub open spec fn HwTriggerOf(idx: int) -> HwTrigger;

pub open spec fn TriggerConfigAt(i: int, s: S) -> TriggerConfig;

pub open spec fn TrigState(idx: int) -> TrigStateVal;

pub open spec fn SavedPrivBits(tdata1: u64) -> TrigStateVal;

pub open spec fn SharedMemWord(offset: int, word: int, s: S) -> int;

pub open spec fn TriggerConfigChains(start: int, end: int) -> Set<TriggerChain>;

pub open spec fn TrigIdxValuesContiguous(chain: TriggerChain, s: S) -> bool;

pub open spec fn HwTriggersContiguous(chain: TriggerChain, s: S) -> bool;

} // verus!
