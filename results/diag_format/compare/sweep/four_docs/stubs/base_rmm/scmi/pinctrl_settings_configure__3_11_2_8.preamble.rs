use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub struct Config {
    pub config_type: u32,
    pub config_value: u32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const INVALID_PARAMETERS: Int32 = -2;
pub spec const DENIED: Int32 = -3;
pub spec const NOT_FOUND: Int32 = -4;
pub spec const PROTOCOL_ERROR: Int32 = -10;
pub spec const IN_USE: Int32 = -12;

pub spec const identifier: u32 = 101;
pub spec const selector: u32 = 102;
pub spec const attributes: u32 = 103;
pub spec const num_configs: u32 = 104;
pub spec const function_id_valid: u32 = 105;
pub spec const function_id: u32 = 106;
pub spec const calling_agent: u32 = 107;
pub spec const configs: Seq<Config> = Seq::empty();

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn IsValidPinOrGroup(s: S, identifier: u32, selector: u32) -> bool;
pub open spec fn Exists(range: bool, cond: bool) -> bool;
pub open spec fn ForAll(range: bool, cond: bool) -> bool;
pub open spec fn IsSortedIncreasingByConfigType(configs: Seq<Config>, num_configs: u32) -> bool;
pub open spec fn TransportMaxConfigs() -> u32;
pub open spec fn IsConfigSupported(s: S, identifier: u32, selector: u32, configs: Seq<Config>, num_configs: u32, function_id_valid: u32, function_id: u32) -> bool;
pub open spec fn AgentMaySetConfig(s: S, calling_agent: u32, identifier: u32, selector: u32) -> bool;
pub open spec fn IsInUseByOtherAgent(s: S, identifier: u32, selector: u32, calling_agent: u32) -> bool;
pub open spec fn PinOrGroupConfig(identifier: u32, selector: u32, config_type: u32) -> u32;
pub open spec fn SelectedFunction(s: S, identifier: u32, selector: u32) -> u32;
pub open spec fn HasFunctionEnabled(s: S, identifier: u32, selector: u32) -> bool;

} // verus!
