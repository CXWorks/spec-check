use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub type UInt8 = u8;

pub struct S {
    pub entities: Seq<UInt32>,
}

pub spec const SUCCESS: Int32 = 0;

pub spec const NOT_FOUND: Int32 = -1;

pub spec const identifier: UInt32 = 0;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn PinctrlEntityExists(entity_type: UInt32, id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn ExtendedName(entity_type: UInt32, id: UInt32) -> [UInt8; 64];

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 64], len: int) -> bool;

} // verus!
