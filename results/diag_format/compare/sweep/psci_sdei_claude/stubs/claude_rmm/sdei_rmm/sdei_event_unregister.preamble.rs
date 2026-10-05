use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type Int64 = i64;

pub struct HandlerInfo {
    pub handler_running: u32,
    pub state: u32,
}

pub struct S {
    pub dummy: u64,
}

pub const TRUE: u32 = 1;

pub const HANDLER_UNREGISTERED: u32 = 0;
pub const HANDLER_REGISTERED: u32 = 1;
pub const HANDLER_UNREGISTER_PENDING: u32 = 2;

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;
pub const PENDING: Int64 = -5;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn IsValidEventNumber(s: S, event: Int32) -> bool;

pub open spec fn EventIsRegisteredByClient(s: S, event: Int32) -> bool;

pub open spec fn EventHandler(s: S, event: Int32) -> HandlerInfo;

pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;

pub open spec fn IsPrivateEvent(s: S, event: Int32) -> bool;

} // verus!
