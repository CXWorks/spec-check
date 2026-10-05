use vstd::prelude::*;
verus! {

pub type Int64 = i64;
pub type ClientId = u64;
pub type PeId = u64;

pub struct S {
    pub pe_masked: Map<PeId, bool>,
}

pub spec const NOT_SUPPORTED: Int64 = (-1int) as i64;
pub spec const SUCCESS: Int64 = 0;

pub spec const client: ClientId = 0;

pub open spec fn SdeiIsSupported() -> bool;
pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;
pub open spec fn PeIsMasked(s: S, c: ClientId, pe: PeId) -> bool;
pub open spec fn CallingPe() -> PeId;
pub open spec fn PendingEventsDispatched(pe: PeId) -> bool;

} // verus!
