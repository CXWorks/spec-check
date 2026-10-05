use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type Int64 = i64;
pub type PeId = u64;
pub type ClientId = u64;
pub type HandlerStateT = u32;

pub struct S {
    pub sdei_supported: bool,
    pub calling_pe: PeId,
    pub calling_client: ClientId,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub const HANDLER_UNREGISTERED: HandlerStateT = 0;
pub const HANDLER_REGISTERED: HandlerStateT = 1;
pub const HANDLER_UNREGISTER_PENDING: HandlerStateT = 2;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;
pub open spec fn IsKnownEvent(s: S, event: Int32) -> bool;
pub open spec fn EventIsRegisteredByClient(s: S, event: Int32) -> bool;
pub open spec fn EventHandlerState(s: S, event: Int32) -> HandlerStateT;
pub open spec fn EventIsPrivate(s: S, event: Int32) -> bool;
pub open spec fn EventIsShared(s: S, event: Int32) -> bool;
pub open spec fn EventIsEnabledForPe(s: S, event: Int32, pe: PeId) -> bool;
pub open spec fn EventIsEnabledForClient(s: S, event: Int32, client: ClientId) -> bool;
pub open spec fn CallingPe(s: S) -> PeId;
pub open spec fn CallingClient(s: S) -> ClientId;

} // verus!
