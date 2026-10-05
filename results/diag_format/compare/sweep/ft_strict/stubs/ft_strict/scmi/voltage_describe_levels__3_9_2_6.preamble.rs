use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const OUT_OF_RANGE: Int32 = -3;
pub const DENIED: Int32 = -4;
pub const NOT_FOUND: Int32 = -5;

pub const result: Int32 = -100;
pub const caller: AgentId = 0;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn VoltageDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidVoltageLevelIndex(s: S, domain_id: UInt32, level_index: UInt32) -> bool;

pub open spec fn IsRequestSupported(s: S, domain_id: UInt32) -> bool;

pub open spec fn AgentMayGetVoltageLevels(s: S, agent: AgentId, domain_id: UInt32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn Elem(arr: [Int32; 4], i: int) -> Int32;

pub open spec fn LowestVoltageLevel(s: S, domain_id: UInt32) -> Int32;

pub open spec fn HighestVoltageLevel(s: S, domain_id: UInt32) -> Int32;

pub open spec fn VoltageStepSize(s: S, domain_id: UInt32) -> Int32;

pub open spec fn VoltageLevelAt(s: S, domain_id: UInt32, index: int) -> Int32;

pub open spec fn RemainingVoltageLevels(s: S, domain_id: UInt32, level_index: UInt32, returned: UInt32) -> UInt32;

} // verus!
