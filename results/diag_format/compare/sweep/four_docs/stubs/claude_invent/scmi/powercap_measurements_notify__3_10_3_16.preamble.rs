use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type ScmiStatusCode = i32;

pub const SUCCESS: ScmiStatusCode = 0;
pub const NOT_SUPPORTED: ScmiStatusCode = -1;
pub const INVALID_PARAMETERS: ScmiStatusCode = -2;
pub const DENIED: ScmiStatusCode = -3;
pub const NOT_FOUND: ScmiStatusCode = -4;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn PowercapMeasurementsThresholdsAreLegal(s: S, domain_id: UInt32, power_thresh_low: UInt32, power_thresh_high: UInt32) -> bool;

pub open spec fn PowercapMeasurementsNotifyEnabled(s: S, agent_id: UInt32, domain_id: UInt32) -> bool;

pub open spec fn PowercapPowerThreshLow(s: S, agent_id: UInt32, domain_id: UInt32) -> UInt32;

pub open spec fn PowercapPowerThreshHigh(s: S, agent_id: UInt32, domain_id: UInt32) -> UInt32;

pub open spec fn PowercapMeasurementsNotifyConfigUnchangedExcept(old_s: S, new_s: S, agent_id: UInt32, domain_id: UInt32) -> bool;

} // verus!
