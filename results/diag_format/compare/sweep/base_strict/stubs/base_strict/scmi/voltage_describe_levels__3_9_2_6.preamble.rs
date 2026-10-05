use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub type Array<T> = Seq<T>;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = (-1int) as i32;
pub spec const OUT_OF_RANGE: Int32 = (-2int) as i32;
pub spec const NOT_SUPPORTED: Int32 = (-3int) as i32;
pub spec const DENIED: Int32 = (-4int) as i32;

pub spec const domain_id: UInt32 = 0;
pub spec const level_index: UInt32 = 0;
pub spec const caller: UInt32 = 0;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn VoltageDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidVoltageLevelIndex(s: S, domain_id: UInt32, level_index: UInt32) -> bool;

pub open spec fn IsRequestSupported(s: S, domain_id: UInt32) -> bool;

pub open spec fn AgentMayGetVoltageLevels(s: S, caller: UInt32, domain_id: UInt32) -> bool;

pub open spec fn Bits64(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn Elem<I>(a: Array<Int32>, i: I) -> Int32;

pub open spec fn LowestVoltageLevel(s: S, domain_id: UInt32) -> Int32;

pub open spec fn HighestVoltageLevel(s: S, domain_id: UInt32) -> Int32;

pub open spec fn VoltageStepSize(s: S, domain_id: UInt32) -> Int32;

pub open spec fn VoltageLevelAt(s: S, domain_id: UInt32, index: int) -> Int32;

pub open spec fn RemainingVoltageLevels(s: S, domain_id: UInt32, level_index: UInt32, count: UInt32) -> UInt32;

} // verus!
