use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub num_data_events_supported: UInt32,
    pub num_event_groups_supported: UInt32,
}

pub open spec fn NumDataEventsSupported() -> UInt32;

pub open spec fn NumEventGroupsSupported() -> UInt32;

pub open spec fn PrimaryDeImplementationRevision() -> u64;

pub open spec fn SupportsSingleSampleAsyncRead() -> bool;

pub open spec fn SupportsContinuousUpdateNotification() -> bool;

pub open spec fn SupportsEventGroupSpecificSampling() -> bool;

pub open spec fn SupportsTelemetryReset() -> bool;

pub open spec fn SupportsFastChannels() -> bool;

pub open spec fn NumShmtisSupported() -> UInt32;

pub open spec fn DefaultBlockTimestampClockRateKhz() -> UInt32;

} // verus!
