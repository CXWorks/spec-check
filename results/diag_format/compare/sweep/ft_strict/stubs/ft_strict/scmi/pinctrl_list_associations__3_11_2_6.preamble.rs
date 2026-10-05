use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub type PinctrlResult = Result<Int32, (Int32, (UInt32, UInt32, [UInt16; 1]))>;

pub struct S {
    pub calling_agent: AgentId,
    pub num_groups: nat,
    pub num_functions: nat,
}

pub spec const SUCCESS: PinctrlResult = Ok(0i32);
pub spec const NOT_SUPPORTED: PinctrlResult = Ok(-1i32);
pub spec const DENIED: PinctrlResult = Ok(-3i32);
pub spec const NOT_FOUND: PinctrlResult = Ok(-4i32);

pub open spec fn GroupOrFunctionExists(s: S, identifier: UInt32, kind: int) -> bool;

pub open spec fn IsRequestSupported(s: S, identifier: UInt32, flags: UInt32, index: UInt32) -> bool;

pub open spec fn AgentMayListAssociations(s: S, agent: AgentId, identifier: UInt32, kind: int) -> bool;

pub open spec fn CallingAgent(s: S) -> AgentId;

pub open spec fn ResultEqual(a: PinctrlResult, b: PinctrlResult) -> bool;

pub open spec fn ArrayLength(array: [UInt16; 1]) -> UInt32;

pub open spec fn ArrayAt(array: [UInt16; 1], i: UInt32) -> UInt16;

pub open spec fn AssociationCount(s: S, identifier: UInt32, kind: int) -> UInt32;

pub open spec fn AssociationAt(s: S, identifier: UInt32, kind: int, idx: UInt32) -> UInt16;

} // verus!
