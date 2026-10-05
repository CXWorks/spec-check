use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: UInt32,
}

pub struct PinConfigEntry {
    pub r#type: UInt32,
    pub config_value: UInt32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const INVALID_PARAMETERS: Int32 = -2;
pub spec const DENIED: Int32 = -3;
pub spec const NOT_FOUND: Int32 = -4;
pub spec const IN_USE: Int32 = -8;
pub spec const PROTOCOL_ERROR: Int32 = -10;

pub spec const identifier: UInt32 = 0;
pub spec const function_id: UInt32 = 1;
pub spec const attributes: UInt32 = 2;
pub spec const calling_agent: UInt32 = 3;
pub spec const configs: Map<UInt32, PinConfigEntry> = Map::empty();

pub open spec fn Bits(value: UInt32, high: UInt32, low: UInt32) -> UInt32;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn IsValidPinOrGroup(identifier: UInt32, selector: UInt32) -> bool;
pub open spec fn AreValidSettingsConfigureParameters(identifier: UInt32, function_id: UInt32, attributes: UInt32, configs: Map<UInt32, PinConfigEntry>) -> bool;
pub open spec fn IsSupportedConfiguration(identifier: UInt32, selector: UInt32, function_id: UInt32, attributes: UInt32, configs: Map<UInt32, PinConfigEntry>) -> bool;
pub open spec fn AgentMaySetConfiguration(agent: UInt32, identifier: UInt32, selector: UInt32) -> bool;
pub open spec fn IsInUseByOtherAgent(identifier: UInt32, selector: UInt32, agent: UInt32) -> bool;
pub open spec fn TransportConfigCapacity() -> UInt32;
pub open spec fn PinOrGroupConfig(identifier: UInt32, selector: UInt32, config_type: UInt32) -> UInt32;
pub open spec fn PinOrGroupFunction(identifier: UInt32, selector: UInt32) -> UInt32;
pub open spec fn PrePinOrGroupFunction(identifier: UInt32, selector: UInt32) -> UInt32;

} // verus!
