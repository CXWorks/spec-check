use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt8 = u8;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub cmd_input_identifier: UInt32,
    pub cmd_input_flags: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn Bits64(value: u64, offset: u64) -> u64;

pub open spec fn EntityExists(identifier: u64, flags: u64) -> bool;

pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

pub open spec fn EntityNameLength(identifier: u64, flags: u64) -> u64;

pub open spec fn ExtendedNameSupported(identifier: u64, flags: u64) -> bool;

} // verus!
