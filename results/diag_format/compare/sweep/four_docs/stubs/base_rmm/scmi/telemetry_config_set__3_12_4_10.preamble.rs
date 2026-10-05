use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type GroupIdentifier = u32;

pub struct TelemetryControl {
    pub enable: u32,
    pub mode: u32,
    pub selector: u32,
    pub reserved: u32,
}

pub struct SamplingRateValue {
    pub sec: int,
    pub exponent: int,
}

pub struct DataEvent {
    pub id: u32,
}

pub struct ProtocolAttributesValue {
    pub attributes_1: Seq<u32>,
}

pub struct S {
    pub group_identifier: GroupIdentifier,
    pub control: TelemetryControl,
    pub sampling_rate: SamplingRateValue,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const INVALID_PARAMETERS: Int32 = -2;
pub spec const OUT_OF_RANGE: Int32 = -7;

pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

pub open spec fn AreValidTelemetryConfigParams(s: S, group_identifier: GroupIdentifier, control: TelemetryControl, sampling_rate: SamplingRateValue) -> bool;

pub open spec fn ProtocolAttributes(s: S) -> ProtocolAttributesValue;

pub open spec fn NoDeEnabled(s: S, selector: u32, group_identifier: GroupIdentifier, control: TelemetryControl) -> bool;

pub open spec fn EnabledDeOrGroupLimitReached(s: S, selector: u32, group_identifier: GroupIdentifier, mode: u32) -> bool;

pub open spec fn TargetDes(s: S, selector: u32, group_identifier: GroupIdentifier) -> Set<DataEvent>;

pub open spec fn TelemetryEnabled(de: DataEvent) -> u32;

pub open spec fn TelemetryMode(de: DataEvent) -> u32;

pub open spec fn InterfaceSupportsOnDemand(de: DataEvent) -> bool;

pub open spec fn SamplingRate(de: DataEvent) -> int;

} // verus!
