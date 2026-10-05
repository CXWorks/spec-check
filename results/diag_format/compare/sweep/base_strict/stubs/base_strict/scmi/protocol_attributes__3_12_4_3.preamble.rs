use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub enum RmiStatusCode {
    Success,
    ErrorInput,
    ErrorRealm,
    ErrorRec,
    ErrorRtt,
    ErrorNotSupported,
    ErrorDenied,
}

pub spec const SUCCESS: RmiStatusCode = RmiStatusCode::Success;

pub struct S {
    pub dummy: int,
}

pub struct DataEvent {
    pub id: u32,
}

pub struct BlockTimestampLine {
    pub id: u32,
}

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn NumDataEventsSupported() -> UInt32;

pub open spec fn NumEventGroupsSupported() -> UInt32;

pub open spec fn DeImplementationRev128(dword0: UInt32, dword1: UInt32, dword2: UInt32, dword3: UInt32) -> u128;

pub open spec fn PrimaryDeImplementationRevision() -> u128;

pub open spec fn Bits(value: UInt32, hi: nat, lo: nat) -> UInt32;

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
