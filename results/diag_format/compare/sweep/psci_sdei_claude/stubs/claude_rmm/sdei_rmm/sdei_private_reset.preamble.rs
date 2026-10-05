use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub type PeId = int;

pub type SdeiEventState = u64;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const DENIED: Int64 = -3;

pub const UNREGISTERED: SdeiEventState = 0;
pub const HANDLER_UNREGISTER_PENDING: SdeiEventState = 1;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn CallingPe(s: S) -> PeId;

pub open spec fn AnyEventHandlerRunning(s: S, pe: PeId) -> bool;

pub open spec fn PrivateEvents(s: S, pe: PeId) -> Set<int>;

pub open spec fn SharedEvents(s: S) -> Set<int>;

pub open spec fn EventIsRunning(s: S, ev: int) -> bool;

pub open spec fn EventState(s: S, ev: int) -> SdeiEventState;

pub open spec fn EventAuxInfoIsCleared(s: S, ev: int) -> bool;

pub open spec fn EventAuxInfoIsWarmBootState(s: S, ev: int) -> bool;

} // verus!
