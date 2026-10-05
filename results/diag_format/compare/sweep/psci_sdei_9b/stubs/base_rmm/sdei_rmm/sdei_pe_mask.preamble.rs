use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type ClientId = u64;
pub type PeId = u64;
pub type Priority = u64;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: Int64 = -1;

pub const NORMAL_PRIORITY: Priority = 0;
pub const CRITICAL_PRIORITY: Priority = 1;

pub open spec fn CallingClient() -> ClientId;

pub open spec fn CallingPe() -> PeId;

pub open spec fn SdeiIsSupported(client: ClientId) -> bool;

pub open spec fn PeIsMasked(client: ClientId, pe: PeId, priority: Priority) -> bool;

pub open spec fn old(b: bool) -> bool;

pub open spec fn ResultEqual(result: Int64, expected: Int64) -> bool;

} // verus!
