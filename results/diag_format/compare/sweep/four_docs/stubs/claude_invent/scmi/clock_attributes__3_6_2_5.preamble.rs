use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: u64,
}

pub open spec fn ClockIsValidForAgent(s: S, clock_id: u32) -> bool;

pub open spec fn ClockRateChangeNotificationsSupported(s: S, clock_id: u32) -> bool;

pub open spec fn ClockRateChangeRequestedNotificationsSupported(s: S, clock_id: u32) -> bool;

pub open spec fn ClockNameLength(s: S, clock_id: u32) -> int;

pub open spec fn ClockParentIdentifiersAdvertised(s: S, clock_id: u32) -> bool;

pub open spec fn ClockExtendedConfigSupported(s: S, clock_id: u32) -> bool;

pub open spec fn ClockHasDiscoverableRestrictions(s: S, clock_id: u32) -> bool;

pub open spec fn ClockIsEnabled(s: S, clock_id: u32) -> bool;

pub open spec fn ClockWorstCaseEnableDelayUs(s: S, clock_id: u32) -> int;

} // verus!
