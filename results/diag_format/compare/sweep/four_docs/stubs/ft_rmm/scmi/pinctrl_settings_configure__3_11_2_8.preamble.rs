use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt8 = u8;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;
pub const IN_USE: Int32 = -8;
pub const PROTOCOL_ERROR: Int32 = -10;

pub const function_id_valid: UInt32 = 0;
pub const calling_agent: UInt32 = 0;
pub const result: Int32 = 0;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidPinOrGroup(s: S, identifier: UInt32, selector: UInt8) -> bool;

pub open spec fn IsSortedIncreasingByConfigType(s: S, configs: [UInt32; 2], num_configs: UInt8) -> bool;

pub open spec fn TransportMaxConfigs() -> UInt8;

pub open spec fn IsConfigSupported(s: S, identifier: UInt32, selector: UInt8, configs: [UInt32; 2], num_configs: UInt8, function_id_valid_arg: UInt32, function_id: UInt32) -> bool;

pub open spec fn AgentMaySetConfig(s: S, agent: UInt32, identifier: UInt32, selector: UInt8) -> bool;

pub open spec fn IsInUseByOtherAgent(s: S, identifier: UInt32, selector: UInt8, agent: UInt32) -> bool;

pub open spec fn PinOrGroupConfig(s: S, identifier: UInt32, selector: UInt8, config_type: UInt32) -> UInt32;

pub open spec fn SelectedFunction(s: S, identifier: UInt32, selector: UInt8) -> UInt32;

pub open spec fn HasFunctionEnabled(s: S, identifier: UInt32, selector: UInt8) -> bool;

} // verus!
