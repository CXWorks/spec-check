use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const NOT_FOUND: Int32 = -3;

pub const SYSTEM_POWER_STATE_SET_ID: UInt32 = 3;
pub const SYSTEM_POWER_STATE_NOTIFY_ID: UInt32 = 5;

pub open spec fn IsMessageImplemented(s: S, message_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

pub open spec fn SystemPowerStateNotificationsSupported(s: S) -> bool;

pub open spec fn SystemWarmResetSupported(s: S) -> bool;

pub open spec fn SystemSuspendSupported(s: S) -> bool;

pub open spec fn Bits(value: UInt32, high: int, low: int) -> int;

} // verus!
