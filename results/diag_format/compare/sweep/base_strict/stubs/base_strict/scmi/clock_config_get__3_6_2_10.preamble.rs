use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub clock_id_val: UInt32,
    pub flags_val: UInt32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = (-1int) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as i32;

pub open spec fn clock_id(s: S) -> UInt32;

pub open spec fn flags(s: S) -> UInt32;

pub open spec fn ClockExists(clock_id: UInt32) -> bool;

pub open spec fn IsValidConfigFlags(clock_id: UInt32, flags: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

pub open spec fn Bits(value: UInt32, high: int, low: int) -> UInt32;

pub open spec fn ClockEnableState(clock_id: UInt32) -> UInt32;

pub open spec fn ExtendedConfigValue(clock_id: UInt32, config_type: UInt32) -> UInt32;

} // verus!
