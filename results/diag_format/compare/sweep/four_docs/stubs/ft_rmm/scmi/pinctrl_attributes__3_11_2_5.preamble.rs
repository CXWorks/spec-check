use vstd::prelude::*;
verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PinctrlObjectExists(s: S, selector: UInt32, identifier: UInt32) -> bool;

pub open spec fn PinctrlNameLength(s: S, selector: UInt32, identifier: UInt32) -> UInt32;

pub open spec fn PinctrlName(s: S, selector: UInt32, identifier: UInt32) -> [UInt8; 16];

pub open spec fn FunctionSupportsGpio(s: S, identifier: UInt32) -> UInt32;

pub open spec fn IsPinOnlyFunction(s: S, identifier: UInt32) -> bool;

pub open spec fn GroupPinCount(s: S, identifier: UInt32) -> UInt32;

pub open spec fn FunctionPinCount(s: S, identifier: UInt32) -> UInt32;

pub open spec fn FunctionGroupCount(s: S, identifier: UInt32) -> UInt32;

} // verus!
