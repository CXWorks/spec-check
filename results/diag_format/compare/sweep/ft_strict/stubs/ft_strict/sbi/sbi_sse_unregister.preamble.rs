use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type Hart = u64;

pub struct S {
    pub dummy: u64,
}

pub const UNUSED: u32 = 0;
pub const REGISTERED: u32 = 1;
pub const ENABLED: u32 = 2;
pub const RUNNING: u32 = 3;

pub open spec fn EventState(s: S, event_id: UInt32, h: Hart) -> u32;

pub open spec fn CallingHart(s: S) -> Hart;

pub open spec fn ResultIsError(error: i64) -> bool;

pub open spec fn ResultIsSuccess(error: i64) -> bool;

pub open spec fn IsLocalEvent(s: S, event_id: UInt32) -> bool;

pub open spec fn IsGlobalEvent(s: S, event_id: UInt32) -> bool;

pub open spec fn HasRegisteredHandler(s: S, event_id: UInt32, h: Hart) -> bool;

} // verus!
