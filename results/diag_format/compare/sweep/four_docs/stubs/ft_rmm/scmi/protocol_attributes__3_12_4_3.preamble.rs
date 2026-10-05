use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub enum RsiCommandReturnCode {
    Success,
    Error,
    ErrorInput,
    ErrorState,
    Incomplete,
}

pub const RSI_SUCCESS: RsiCommandReturnCode = RsiCommandReturnCode::Success;

pub struct S {
    pub dummy: int,
}

pub open spec fn NumDataEventsSupported() -> UInt32;

pub open spec fn NumEventGroupsSupported() -> UInt32;

pub open spec fn PrimaryDeImplementationRevision() -> UInt32;

pub open spec fn SupportsSingleSampleAsyncRead() -> bool;

pub open spec fn SupportsContinuousUpdateNotification() -> bool;

pub open spec fn SupportsEventGroupSpecificSampling() -> bool;

pub open spec fn SupportsTelemetryReset() -> bool;

pub open spec fn SupportsFastChannels() -> bool;

pub open spec fn NumShmtisSupported() -> UInt32;

pub open spec fn DefaultBlockTimestampClockRateKhz() -> UInt32;

} // verus!
