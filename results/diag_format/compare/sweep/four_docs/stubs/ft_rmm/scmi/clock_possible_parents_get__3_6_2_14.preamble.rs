use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub clocks: Seq<UInt32>,
    pub agents: Seq<AgentId>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const OUT_OF_RANGE: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;
pub const DENIED: Int32 = -5;

pub const N: UInt32 = 1;
pub const calling_agent: AgentId = 0;

pub open spec fn ClockExists(s: S, clock_id: UInt32) -> bool;
pub open spec fn IsValidParentSkip(s: S, clock_id: UInt32, skip_parents: UInt32) -> bool;
pub open spec fn IsPossibleParentsQuerySupported(s: S, clock_id: UInt32) -> bool;
pub open spec fn AgentMayGetPossibleParents(s: S, agent: AgentId, clock_id: UInt32) -> bool;
pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;
pub open spec fn RemainingPossibleParents(s: S, clock_id: UInt32, skip_parents: UInt32, n: UInt32) -> UInt32;
pub open spec fn PossibleParents(s: S, clock_id: UInt32) -> Seq<UInt32>;
pub open spec fn IsAscendingOrder(s: S, parents: Seq<UInt32>) -> bool;

} // verus!
