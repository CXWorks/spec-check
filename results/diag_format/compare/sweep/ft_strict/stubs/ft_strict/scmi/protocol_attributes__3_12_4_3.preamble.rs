use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub const SUCCESS: Int32 = 0;

pub struct S {
    pub dummy: int,
}

pub struct DataEvent {
    pub id: UInt32,
}

pub struct BlockTimestampLine {
    pub id: UInt32,
}

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn NumDataEventsSupported() -> UInt32;

pub open spec fn NumEventGroupsSupported() -> UInt32;

pub open spec fn DeImplementationRev128(d0: UInt32, d1: UInt32, d2: UInt32, d3: UInt32) -> u128;

pub open spec fn PrimaryDeImplementationRevision() -> u128;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn SupportsSingleSampleAsyncReadViaConfigSet() -> bool;

pub open spec fn ReadCompletionSignalledByTelemetryReadingComplete() -> bool;

pub open spec fn SupportsContinuousUpdateNotifications() -> bool;

pub open spec fn TelemetryUpdateSentAtSamplingInterval() -> bool;

pub open spec fn PerEventGroupSamplingRateAndCollectionModeConfigurable() -> bool;

pub open spec fn UngroupedDesShareSamplingRateAndCollectionMode() -> bool;

pub open spec fn AllDesAndGroupsShareSamplingRateAndCollectionMode() -> bool;

pub open spec fn SupportsFullTelemetryReset() -> bool;

pub open spec fn FastChannelsSupported() -> bool;

pub open spec fn UsesFastChannel(de: DataEvent) -> bool;

pub open spec fn NumShmtisSupported() -> UInt32;

pub open spec fn DefaultBlockTimestampClockRateKhz() -> UInt32;

pub open spec fn BlockTimestampClockRateKhz(ts: BlockTimestampLine) -> UInt32;

} // verus!
