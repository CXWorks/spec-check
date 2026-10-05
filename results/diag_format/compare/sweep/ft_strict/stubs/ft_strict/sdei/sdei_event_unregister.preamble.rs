use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type PeId = u64;

pub struct S {
    pub dummy: int,
}

pub enum SdeiStatusCode {
    Success,
    NotSupported,
    InvalidParameters,
    Denied,
    Pending,
    OutOfResource,
}

impl SdeiStatusCode {
    pub const SUCCESS: Result<(), SdeiStatusCode> = Ok(());
}

pub const SUCCESS: SdeiStatusCode = SdeiStatusCode::Success;
pub const NOT_SUPPORTED: SdeiStatusCode = SdeiStatusCode::NotSupported;
pub const INVALID_PARAMETERS: SdeiStatusCode = SdeiStatusCode::InvalidParameters;
pub const DENIED: SdeiStatusCode = SdeiStatusCode::Denied;
pub const PENDING: SdeiStatusCode = SdeiStatusCode::Pending;

pub open spec fn IsSdeiSupported(s: S) -> bool;

pub open spec fn ResultEqual(result: Result<(), SdeiStatusCode>, code: SdeiStatusCode) -> bool;

pub open spec fn IsValidEventNumber(s: S, event: Int32) -> bool;

pub open spec fn IsEventRegisteredByClient(s: S, event: Int32) -> bool;

pub open spec fn IsHandlerRunning(s: S, event: Int32) -> bool;

pub open spec fn IsUnregisterPending(s: S, event: Int32) -> bool;

pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;

pub open spec fn IsPrivateEvent(s: S, event: Int32) -> bool;

pub open spec fn IsEventRegisteredGlobally(s: S, event: Int32) -> bool;

pub open spec fn IsEventRegisteredOnPe(s: S, event: Int32, pe: PeId) -> bool;

pub open spec fn CurrentPe() -> PeId;

pub open spec fn IsEventDeliverableToClient(s: S, event: Int32) -> bool;

} // verus!
