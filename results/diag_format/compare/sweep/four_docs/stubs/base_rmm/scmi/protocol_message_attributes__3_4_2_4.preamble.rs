use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub msg_id: UInt32,
    pub power_state_notifications_supported: bool,
    pub system_warm_reset_supported: bool,
    pub system_suspend_supported: bool,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const NOT_FOUND: Int32 = -3;

pub const SYSTEM_POWER_STATE_SET_ID: UInt32 = 3;
pub const SYSTEM_POWER_STATE_NOTIFY_ID: UInt32 = 5;

pub open spec fn message_id(s: S) -> UInt32;

pub open spec fn IsImplementedMessage(id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PowerStateNotificationsSupported(s: S) -> bool;

pub open spec fn SystemWarmResetSupported(s: S) -> bool;

pub open spec fn SystemSuspendSupported(s: S) -> bool;

} // verus!
