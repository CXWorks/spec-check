use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type RmiStatusCode = u64;

pub struct S {
    pub dummy: UInt64,
}

pub spec const RMI_SUCCESS: RmiStatusCode = 0;
pub spec const RMI_ERROR_NOT_FOUND: RmiStatusCode = 1;
pub spec const RMI_ERROR_INVALID_PARAMETERS: RmiStatusCode = 2;

pub spec const clock_id: UInt64 = 100;
pub spec const calling_agent: UInt64 = 200;
pub spec const notify_enable: Seq<UInt64> = Seq::<UInt64>::empty();

pub open spec fn IsValidClockDevice(s: S, id: UInt64) -> bool;

pub open spec fn IsValidNotifyEnable(s: S, ne: Seq<UInt64>) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn ClockRateNotifyEnabled(s: S, agent: UInt64, id: UInt64) -> bool;

} // verus!
