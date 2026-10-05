use vstd::prelude::*;

verus! {

pub type TrigIdx = u64;

pub const XLEN: u64 = 64;

pub struct S {
    pub dummy: u64,
}

pub struct Tdata1 {
    pub vs: bool,
    pub vu: bool,
    pub s: bool,
    pub u: bool,
    pub raw: u64,
}

pub struct TrigConfig {
    pub tdata1: Tdata1,
    pub tdata2: u64,
    pub tdata3: u64,
}

pub struct HwTrigger {
    pub id: u64,
    pub tdata1: Tdata1,
    pub tdata2: u64,
    pub tdata3: u64,
}

pub struct TrigStateRec {
    pub vs: bool,
    pub vu: bool,
    pub s: bool,
    pub u: bool,
}

pub struct TriggerChain {
    pub start: u64,
    pub len: u64,
}

pub open spec fn TriggerConfigsProcessedInIncreasingIndexOrderFromZero(trig_count: u64) -> spec_fn(S) -> bool;

pub open spec fn PreTrigIdxInUse(idx: TrigIdx) -> spec_fn(S) -> bool;

pub open spec fn PreHwTriggerInUse(t: HwTrigger) -> spec_fn(S) -> bool;

pub open spec fn InstalledTrigIdx(i: u64) -> TrigIdx;

pub open spec fn HwTriggerOf(idx: TrigIdx) -> HwTrigger;

pub open spec fn HwTriggerMatchesConfig(t: HwTrigger, c: TrigConfig) -> spec_fn(S) -> bool;

pub open spec fn TrigConfigAt(i: u64) -> TrigConfig;

pub open spec fn TrigState(idx: TrigIdx) -> TrigStateRec;

pub open spec fn SharedMemWordLE(offset: int, word: int) -> TrigIdx;

pub open spec fn IsTriggerChainInSharedMem(c: TriggerChain, trig_count: u64) -> bool;

pub open spec fn TrigIdxsContiguous(c: TriggerChain) -> bool;

pub open spec fn HwTriggersContiguous(c: TriggerChain) -> bool;

} // verus!
