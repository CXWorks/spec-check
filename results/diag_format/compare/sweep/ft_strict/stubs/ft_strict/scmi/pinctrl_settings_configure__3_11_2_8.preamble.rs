use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const INVALID_PARAMETERS: Int32 = -2;
pub spec const DENIED: Int32 = -3;
pub spec const NOT_FOUND: Int32 = -4;
pub spec const IN_USE: Int32 = -6;
pub spec const PROTOCOL_ERROR: Int32 = -10;
pub spec const result: Int32 = -100;

pub spec const calling_agent: UInt32 = 0;

pub open spec fn Bits(value: UInt32, hi: UInt32, lo: UInt32) -> UInt32;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidPinOrGroup(s: S, identifier: UInt32, selector: UInt32) -> bool;

pub open spec fn AreValidSettingsConfigureParameters(s: S, identifier: UInt32, function_id: UInt32, attributes: UInt32, configs: [UInt32; 2]) -> bool;

pub open spec fn IsSupportedConfiguration(s: S, identifier: UInt32, selector: UInt32, function_id: UInt32, attributes: UInt32, configs: [UInt32; 2]) -> bool;

pub open spec fn AgentMaySetConfiguration(s: S, agent: UInt32, identifier: UInt32, selector: UInt32) -> bool;

pub open spec fn IsInUseByOtherAgent(s: S, identifier: UInt32, selector: UInt32, agent: UInt32) -> bool;

pub open spec fn TransportConfigCapacity() -> UInt32;

pub open spec fn PinOrGroupConfig(s: S, identifier: UInt32, selector: UInt32, config_type: UInt32) -> UInt32;

pub open spec fn PinOrGroupFunction(s: S, identifier: UInt32, selector: UInt32) -> UInt32;

pub open spec fn PrePinOrGroupFunction(s: S, identifier: UInt32, selector: UInt32) -> UInt32;

} // verus!
