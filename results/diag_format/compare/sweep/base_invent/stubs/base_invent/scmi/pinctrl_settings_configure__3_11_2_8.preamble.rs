use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type UInt32 = u32;
pub type Configs = Seq<u32>;

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const INVALID_PARAMETERS: int32 = -2;
pub const DENIED: int32 = -3;
pub const NOT_FOUND: int32 = -4;
pub const IN_USE: int32 = -6;
pub const PROTOCOL_ERROR: int32 = -10;

pub struct S {
    pub pin_or_group_configs: Map<u32, Seq<u32>>,
    pub pin_or_group_function_selection: Map<u32, u32>,
    pub pin_or_group_attributes: Map<u32, u32>,
    pub pin_or_group_in_use: Map<u32, bool>,
    pub agent_permissions: Map<u32, bool>,
    pub transport_capacity: int,
}

pub spec const identifier: UInt32 = 1;
pub spec const attributes_bits_31_11: UInt32 = 2;
pub spec const num_configs: UInt32 = 3;
pub spec const function_id_valid: UInt32 = 4;
pub spec const attributes_bits_9_2: UInt32 = 5;
pub spec const attributes_bits_1_0: UInt32 = 6;
pub spec const function_id: UInt32 = 7;
pub spec const transport_max_configs: int = 8;
pub spec const configs: Configs = Seq::empty();

pub open spec fn IsPinOrGroupValid(s: S, id: UInt32) -> bool;
pub open spec fn IsConfigSupported(s: S, id: UInt32, fid: UInt32, cfgs: Configs) -> bool;
pub open spec fn IsAgentAllowedToSetConfig(s: S, id: UInt32) -> bool;
pub open spec fn IsPinOrGroupInUse(s: S, id: UInt32) -> bool;

} // verus!
