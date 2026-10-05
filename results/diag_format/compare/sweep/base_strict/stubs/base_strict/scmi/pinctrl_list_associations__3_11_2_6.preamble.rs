use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type AgentId = u32;
pub type Array<T> = Seq<T>;

pub struct S {
    pub agent: AgentId,
    pub state_id: nat,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const NOT_FOUND: Int32 = -3;
pub spec const DENIED: Int32 = -4;

pub spec const identifier: UInt32 = 0;
pub spec const flags: UInt32 = 1;
pub spec const index: UInt32 = 2;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn Bits64(value: UInt32, hi: int, lo: int) -> UInt64;

pub open spec fn GroupOrFunctionExists(id: UInt32, selector: int) -> bool;

pub open spec fn IsRequestSupported(id: UInt32, f: UInt32, idx: UInt32) -> bool;

pub open spec fn CallingAgent() -> AgentId;

pub open spec fn AgentMayListAssociations(agent: AgentId, id: UInt32, selector: int) -> bool;

pub open spec fn ArrayLength(a: Array<UInt16>) -> nat;

pub open spec fn ArrayAt<I>(a: Array<UInt16>, i: I) -> UInt16;

pub open spec fn AssociationCount(id: UInt32, selector: int) -> UInt32;

pub open spec fn AssociationAt(id: UInt32, selector: int, pos: int) -> UInt16;

} // verus!
