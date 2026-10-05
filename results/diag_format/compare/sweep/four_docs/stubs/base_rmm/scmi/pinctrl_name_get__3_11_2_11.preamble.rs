use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt8 = u8;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = -2;

pub spec const selector: UInt32 = 1;
pub spec const identifier: UInt32 = 2;

pub open spec fn PinctrlEntityExists(selector: UInt32, identifier: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PinctrlExtendedName(selector: UInt32, identifier: UInt32) -> [UInt8; 64];

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 64], len: int) -> bool;

} // verus!
