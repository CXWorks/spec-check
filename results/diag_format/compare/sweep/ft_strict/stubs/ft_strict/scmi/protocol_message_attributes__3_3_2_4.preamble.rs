use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub const POWER_STATE_NOTIFY: UInt32 = 5;
pub const POWER_STATE_CHANGE_REQUESTED_NOTIFY: UInt32 = 6;

pub open spec fn IsValidMessageId(s: S, message_id: UInt32) -> bool;

pub open spec fn IsMessageImplemented(s: S, message_id: UInt32) -> bool;

pub open spec fn IsMessageAvailableToCaller(s: S, message_id: UInt32) -> bool;

pub open spec fn PowerStateNotificationsSupported(s: S, message_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
