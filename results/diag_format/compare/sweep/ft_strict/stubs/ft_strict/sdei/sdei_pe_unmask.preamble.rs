use vstd::prelude::*;
verus! {

pub enum SdeiStatusCode {
    NotSupported,
    InvalidParameters,
    Denied,
    Pending,
    OutOfResource,
}

pub type Pe = u64;
pub type Event = u64;
pub type Client = u64;
pub type Priority = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: Result<(), SdeiStatusCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), SdeiStatusCode> = Err(SdeiStatusCode::NotSupported);

pub spec const NORMAL: Priority = 0;
pub spec const CRITICAL: Priority = 1;

pub open spec fn SdeiImplementedForClient(s: S, c: Client) -> bool;
pub open spec fn CallingClient() -> Client;
pub open spec fn CallingPe() -> Pe;
pub open spec fn ResultEqual(r1: Result<(), SdeiStatusCode>, r2: Result<(), SdeiStatusCode>) -> bool;
pub open spec fn PeMaskedForPriority(s: S, c: Client, pe: Pe, p: Priority) -> bool;
pub open spec fn PeMaskStateUnchanged(s: S, c: Client, pe: Pe) -> bool;
pub open spec fn EventStatusUnchanged(s: S, ev: Event) -> bool;
pub open spec fn PendingEventsDispatched(s: S, c: Client, pe: Pe) -> bool;

} // verus!
