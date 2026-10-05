use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub implemented_messages: Set<UInt32>,
    pub available_notifications: Set<UInt32>,
    pub supported_notifications: Set<UInt32>,
    pub fast_channel_messages: Set<UInt32>,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = -4;

pub spec const result: Int32 = -99;

pub spec const POWERCAP_CAP_NOTIFY: UInt32 = 0xA;
pub spec const POWERCAP_MEASUREMENTS_NOTIFY: UInt32 = 0xB;

pub open spec fn IsMessageImplemented(s: S, message_id: UInt32) -> bool;

pub open spec fn IsNotificationAvailableToAgent(s: S, message_id: UInt32) -> bool;

pub open spec fn IsNotificationSupported(s: S, message_id: UInt32) -> bool;

pub open spec fn HasDedicatedFastChannel(s: S, message_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

} // verus!
