use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub type Client = u64;

pub type Pe = u64;

pub type Event = u64;

pub type Priority = u8;

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: Int64 = 0;

pub const NOT_SUPPORTED: Int64 = -1;

pub const NORMAL: Priority = 0;

pub const CRITICAL: Priority = 1;

pub open spec fn SdeiImplementedForClient(s: S, client: Client) -> bool;

pub open spec fn CallingClient() -> Client;

pub open spec fn CallingPe() -> Pe;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn PeMaskedForPriority(s: S, client: Client, pe: Pe, priority: Priority) -> bool;

pub open spec fn PeMaskStateUnchanged(s: S, client: Client, pe: Pe) -> bool;

pub open spec fn EventStatusUnchanged(ev: Event) -> bool;

pub open spec fn PendingEventsDispatched(s: S, client: Client, pe: Pe) -> bool;

} // verus!
