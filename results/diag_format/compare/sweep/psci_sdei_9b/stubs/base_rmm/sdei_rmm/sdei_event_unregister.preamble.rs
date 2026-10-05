use vstd::prelude::*;
verus! {

pub type Int64 = i64;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub struct EventHandlerInfo {
    pub handler_running: bool,
    pub state: u32,
}

pub const TRUE: bool = true;

pub const HANDLER_UNREGISTERED: u32 = 0;
pub const HANDLER_REGISTERED: u32 = 1;
pub const HANDLER_UNREGISTER_PENDING: u32 = 2;

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;
pub const PENDING: Int64 = -5;

pub open spec fn SdeiIsSupported() -> bool;
pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;
pub open spec fn IsValidEventNumber(event: Int32) -> bool;
pub open spec fn EventIsRegisteredByClient(event: Int32) -> bool;
pub open spec fn EventHandler(event: Int32) -> EventHandlerInfo;
pub open spec fn IsSharedEvent(event: Int32) -> bool;
pub open spec fn IsPrivateEvent(event: Int32) -> bool;
pub open spec fn NoFurtherEventsDelivered(event: Int32) -> bool;

} // verus!
