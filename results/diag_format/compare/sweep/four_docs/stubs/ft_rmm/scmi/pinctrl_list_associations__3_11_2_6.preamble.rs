use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct PinctrlListAssociationsFlags {
    pub selector: UInt32,
    pub reserved: UInt32,
}

pub struct PinctrlListAssociationsFlags1 {
    pub num_returned: UInt32,
    pub remaining: UInt32,
    pub reserved: UInt32,
}

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const NOT_FOUND: Int32 = -4;
pub const DENIED: Int32 = -3;

pub const result: Int32 = -100;

pub const calling_agent: UInt32 = 0;

pub open spec fn GroupOrFunctionExists(s: S, identifier: UInt32, selector: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsRequestSupported(s: S, identifier: UInt32, flags: PinctrlListAssociationsFlags, index: UInt32) -> bool;

pub open spec fn AgentMayListAssociations(s: S, agent: UInt32, identifier: UInt32, selector: UInt32) -> bool;

pub open spec fn NumRemainingAssociations(s: S, identifier: UInt32, selector: UInt32, index: UInt32, returned: UInt32) -> UInt32;

pub open spec fn AssociatedIdentifiers(s: S, identifier: UInt32, selector: UInt32) -> Seq<UInt16>;

pub open spec fn IsAscending(s: S, ids: Seq<UInt16>) -> bool;

} // verus!
