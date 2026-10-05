use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0i32;
pub const NOT_SUPPORTED: i32 = -1i32;
pub const NOT_FOUND: i32 = -4i32;
pub const DENIED: i32 = -3i32;

pub open spec fn PinctrlGroupExists(s: S, identifier: UInt32) -> bool;

pub open spec fn PinctrlFunctionExists(s: S, identifier: UInt32) -> bool;

pub open spec fn PinctrlListAssociationsSupported(s: S, identifier: UInt32, flags: UInt32, index: UInt32) -> bool;

pub open spec fn PinctrlAgentAllowedListAssociations(s: S, identifier: UInt32, selector: UInt32) -> bool;

pub open spec fn PinctrlAssociations(s: S, identifier: UInt32, selector: UInt32) -> Seq<u16>;

} // verus!
