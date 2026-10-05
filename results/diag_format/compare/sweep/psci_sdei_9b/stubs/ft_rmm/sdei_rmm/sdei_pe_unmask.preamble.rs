use vstd::prelude::*;
verus! {

pub enum SdeiStatusCode {
    NotSupported,
    InvalidParameters,
    Denied,
    Pending,
    OutOfResource,
}

pub type ClientId = u64;
pub type PeId = u64;

pub struct S {
    pub supported: bool,
    pub current_client: ClientId,
    pub calling_pe: PeId,
}

pub spec const SUCCESS: Result<(), SdeiStatusCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), SdeiStatusCode> = Err(SdeiStatusCode::NotSupported);

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn ResultEqual(a: Result<(), SdeiStatusCode>, b: Result<(), SdeiStatusCode>) -> bool;
pub open spec fn PeIsMasked(s: S, c: ClientId, pe: PeId) -> bool;
pub open spec fn client(s: S) -> ClientId;
pub open spec fn CallingPe(s: S) -> PeId;
pub open spec fn PendingEventsDispatched(s: S, pe: PeId) -> bool;

} // verus!
