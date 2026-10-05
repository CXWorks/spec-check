use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;

pub struct PinControlFlags {
    pub selector: UInt32,
    pub num_returned: UInt32,
    pub remaining: UInt32,
    pub reserved: UInt32,
}

pub struct S {
    pub identifier: UInt32,
    pub index: UInt32,
    pub calling_agent: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const NOT_FOUND: Int32 = -4;
pub const DENIED: Int32 = -3;

pub const N: UInt32 = 8;

pub open spec fn identifier(s: S) -> UInt32;

pub open spec fn index(s: S) -> UInt32;

pub open spec fn calling_agent(s: S) -> UInt32;

pub open spec fn GroupOrFunctionExists(id: UInt32, selector: UInt32) -> bool;

pub open spec fn IsRequestSupported(id: UInt32, flags: PinControlFlags, idx: UInt32) -> bool;

pub open spec fn AgentMayListAssociations(agent: UInt32, id: UInt32, selector: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn NumRemainingAssociations(id: UInt32, selector: UInt32, idx: UInt32, n: UInt32) -> UInt32;

pub open spec fn AssociatedIdentifiers(id: UInt32, selector: UInt32) -> Seq<UInt16>;

pub open spec fn IsAscending(a: Seq<UInt16>) -> bool;

} // verus!
