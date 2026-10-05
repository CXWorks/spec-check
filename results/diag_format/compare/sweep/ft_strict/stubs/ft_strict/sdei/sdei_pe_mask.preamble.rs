use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub type Pe = u64;

pub type Client = u64;

pub type SdeiEvent = u64;

pub type SdeiPriority = u32;

pub type SdeiEventStatusValue = u32;

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: Int64 = -1;

pub const SDEI_PRIORITY_NORMAL: SdeiPriority = 1;

pub const SDEI_PRIORITY_CRITICAL: SdeiPriority = 0;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(result: Int64, expected: Int64) -> bool;

pub open spec fn CallingClient() -> Client;

pub open spec fn CallingPe() -> Pe;

pub open spec fn PeSdeiMasked(s: S, c: Client, p: Pe) -> bool;

pub open spec fn PeSdeiMaskedBeforeCall(s: S, c: Client, p: Pe) -> bool;

pub open spec fn PeCanReceiveSdeiEvent(s: S, c: Client, p: Pe, prio: SdeiPriority) -> bool;

pub open spec fn SdeiEventStatus(s: S, e: SdeiEvent) -> SdeiEventStatusValue;

pub open spec fn SdeiEventStatusBeforeCall(s: S, e: SdeiEvent) -> SdeiEventStatusValue;

} // verus!
