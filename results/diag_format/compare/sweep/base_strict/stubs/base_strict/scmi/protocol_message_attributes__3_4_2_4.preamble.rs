use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const NOT_FOUND: Int32 = -4;

pub const SYSTEM_POWER_STATE_SET_ID: UInt32 = 3;
pub const SYSTEM_POWER_STATE_NOTIFY_ID: UInt32 = 5;

pub open spec fn IsMessageImplemented(message_id: UInt32) -> bool;

pub open spec fn SystemPowerStateNotificationsSupported() -> bool;

pub open spec fn SystemWarmResetSupported() -> bool;

pub open spec fn SystemSuspendSupported() -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
