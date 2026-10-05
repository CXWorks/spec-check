use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub implemented_messages: Set<u32>,
    pub system_power_state_notifications_supported: bool,
    pub system_warm_reset_supported: bool,
    pub system_suspend_supported: bool,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const NOT_FOUND: i32 = -3;

pub const SYSTEM_POWER_STATE_SET: u32 = 0x3;
pub const SYSTEM_POWER_STATE_NOTIFY: u32 = 0x5;

pub open spec fn IsMessageImplemented(s: S, message_id: UInt32) -> bool;

pub open spec fn SystemPowerStateNotificationsSupported(s: S) -> bool;

pub open spec fn SystemWarmResetSupported(s: S) -> bool;

pub open spec fn SystemSuspendSupported(s: S) -> bool;

} // verus!
