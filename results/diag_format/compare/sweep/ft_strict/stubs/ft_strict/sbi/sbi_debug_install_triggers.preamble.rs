use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type long = i64;

pub const XLEN: u64 = 64;

pub struct Tdata1 {
    pub vs: bool,
    pub vu: bool,
    pub s: bool,
    pub u: bool,
}

pub struct TrigConfig {
    pub tdata1: Tdata1,
    pub tdata2: UInt64,
    pub tdata3: UInt64,
}

pub struct TrigStateT {
    pub vs: bool,
    pub vu: bool,
    pub s: bool,
    pub u: bool,
}

pub struct HwTrigger {
    pub tdata1: Tdata1,
    pub tdata2: UInt64,
    pub tdata3: UInt64,
}

pub struct TriggerChain {
    pub start: UInt64,
    pub len: UInt64,
}

pub struct S {
    pub shared_mem: Seq<u8>,
    pub trig_count: UInt64,
}

pub open spec fn TriggerConfigsProcessedInIncreasingIndexOrderFromZero(s: S, trig_count: UInt64) -> bool;

pub open spec fn InstalledTrigIdx(i: UInt64) -> UInt64;

pub open spec fn HwTriggerOf(s: S, idx: UInt64) -> HwTrigger;

pub open spec fn PreTrigIdxInUse(s: S, idx: UInt64) -> bool;

pub open spec fn PreHwTriggerInUse(s: S, hw: HwTrigger) -> bool;

pub open spec fn HwTriggerMatchesConfig(s: S, hw: HwTrigger, cfg: TrigConfig) -> bool;

pub open spec fn TrigConfigAt(s: S, i: UInt64) -> TrigConfig;

pub open spec fn TrigState(s: S, idx: UInt64) -> TrigStateT;

pub open spec fn SharedMemWordLE(s: S, offset: int, word: int) -> UInt64;

pub open spec fn IsTriggerChainInSharedMem(s: S, c: TriggerChain, trig_count: UInt64) -> bool;

pub open spec fn TrigIdxsContiguous(s: S, c: TriggerChain) -> bool;

pub open spec fn HwTriggersContiguous(s: S, c: TriggerChain) -> bool;

} // verus!
