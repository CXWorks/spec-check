use vstd::prelude::*;

verus! {

pub type RmiStatusCode = u64;

pub type UInt64 = u64;

pub struct Agent {
    pub id: u64,
}

pub struct S {
    pub dummy: u64,
}

pub const RMI_SUCCESS: RmiStatusCode = 0;
pub const RMI_ERROR_INPUT: RmiStatusCode = 1;

pub const notify_enable: UInt64 = 0;

pub open spec fn IsValidNotifyEnable(v: UInt64) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn Bits(v: UInt64, hi: int, lo: int) -> int;

pub open spec fn CallingAgent() -> Agent;

pub open spec fn ErrorEventNotificationsEnabled(a: Agent) -> bool;

} // verus!
