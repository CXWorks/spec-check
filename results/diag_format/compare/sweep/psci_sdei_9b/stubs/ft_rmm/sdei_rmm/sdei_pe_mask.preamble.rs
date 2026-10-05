use vstd::prelude::*;

verus! {

pub type SdeiStatusCode = i64;

pub type ClientId = u64;

pub type PeId = u64;

pub type Priority = u64;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: SdeiStatusCode = -1;

pub const NORMAL_PRIORITY: Priority = 0;

pub const CRITICAL_PRIORITY: Priority = 1;

pub open spec fn CallingClient() -> ClientId;

pub open spec fn CallingPe() -> PeId;

pub open spec fn SdeiIsSupported(s: S, client: ClientId) -> bool;

pub open spec fn PeIsMasked(s: S, client: ClientId, pe: PeId, priority: Priority) -> bool;

pub open spec fn ResultEqual(result: Result<(), SdeiStatusCode>, code: SdeiStatusCode) -> bool;

} // verus!
