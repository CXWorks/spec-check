use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type Array<T> = Seq<T>;
pub type ClockId = u32;
pub type AgentId = u32;

pub struct S {
    pub clock_id: ClockId,
    pub skip_parents: int,
    pub calling_agent: AgentId,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const OUT_OF_RANGE: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;
pub const DENIED: Int32 = -5;

pub open spec fn clock_id(s: S) -> ClockId;
pub open spec fn skip_parents(s: S) -> int;
pub open spec fn calling_agent(s: S) -> AgentId;
pub open spec fn ClockExists(s: S, clock: ClockId) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn IsValidParentSkip(s: S, clock: ClockId, skip: int) -> bool;
pub open spec fn IsPossibleParentsQuerySupported(s: S, clock: ClockId) -> bool;
pub open spec fn AgentMayGetPossibleParents(s: S, agent: AgentId, clock: ClockId) -> bool;
pub open spec fn N(s: S, clock: ClockId, skip: int) -> int;
pub open spec fn RemainingPossibleParents(s: S, clock: ClockId, skip: int, n: int) -> int;
pub open spec fn PossibleParents(s: S, clock: ClockId) -> Seq<UInt32>;
pub open spec fn IsAscendingOrder(a: Seq<UInt32>) -> bool;

} // verus!
