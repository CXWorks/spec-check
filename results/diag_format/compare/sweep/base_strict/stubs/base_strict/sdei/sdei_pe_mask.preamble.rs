use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub type UInt32 = u32;

pub type UInt64 = u64;

pub struct S {
    pub dummy: u64,
}

pub type Client = S;

pub type Pe = u64;

pub type SdeiEvent = u32;

pub type SdeiPriority = u32;

pub type SdeiEventStatusValue = u64;

pub const NOT_SUPPORTED: Int64 = -1;

pub const SDEI_PRIORITY_NORMAL: SdeiPriority = 0;

pub const SDEI_PRIORITY_CRITICAL: SdeiPriority = 1;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(result: Int64, expected: Int64) -> bool;

pub open spec fn CallingClient() -> Client;

pub open spec fn CallingPe() -> Pe;

pub open spec fn PeSdeiMasked(a: S, b: Client, p: Pe) -> bool;

pub open spec fn PeSdeiMaskedBeforeCall(s: S, c: Client, p: Pe) -> bool;

pub open spec fn PeCanReceiveSdeiEvent(s: S, c: Client, p: Pe, prio: SdeiPriority) -> bool;

pub open spec fn SdeiEventStatus(e: SdeiEvent, s: S) -> SdeiEventStatusValue;

pub open spec fn SdeiEventStatusBeforeCall(e: SdeiEvent, s: S) -> SdeiEventStatusValue;

} // verus!
