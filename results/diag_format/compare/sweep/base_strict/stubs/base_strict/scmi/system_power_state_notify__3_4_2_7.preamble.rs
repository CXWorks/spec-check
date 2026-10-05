use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt64 = u64;

pub struct S {
    pub cmd_input_notify_enable: UInt64,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2) as i32;

pub open spec fn SystemPowerStateNotifySupported(s: S) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn Bits64(value: UInt64, hi: int, lo: int) -> UInt64;

pub open spec fn IsPermissibleNotifyEnable(s: S, value: UInt64) -> bool;

pub open spec fn SystemPowerStateNotifyEnabled(s: S) -> bool;

} // verus!
