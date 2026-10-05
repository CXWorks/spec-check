use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub type ClientId = u64;

pub type PeId = u64;

pub type Priority = u32;

pub struct S {
    pub calling_client: ClientId,
    pub calling_pe: PeId,
}

pub const NOT_SUPPORTED: Int64 = -1;

pub const NORMAL_PRIORITY: Priority = 0;

pub const CRITICAL_PRIORITY: Priority = 1;

pub open spec fn SdeiIsSupported(s: S, client: ClientId) -> bool;

pub open spec fn CallingClient(s: S) -> ClientId;

pub open spec fn CallingPe(s: S) -> PeId;

pub open spec fn PeIsMasked(s: S, client: ClientId, pe: PeId, priority: Priority) -> bool;

pub open spec fn ResultEqual(result: Int64, expected: Int64) -> bool;

} // verus!
