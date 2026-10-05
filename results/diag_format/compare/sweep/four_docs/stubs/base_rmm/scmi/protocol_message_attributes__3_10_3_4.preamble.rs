use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub const POWERCAP_CAP_NOTIFY: UInt32 = 0x0B;
pub const POWERCAP_MEASUREMENTS_NOTIFY: UInt32 = 0x0C;

pub open spec fn IsMessageImplemented(message_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn IsNotificationAvailableToAgent(message_id: UInt32) -> bool;

pub open spec fn HasDedicatedFastChannel(message_id: UInt32) -> bool;

pub open spec fn IsNotificationSupported(message_id: UInt32) -> bool;

} // verus!
