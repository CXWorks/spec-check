use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub type TelemetryDeEnabledEntry = (uint32, uint32);

pub type TelemetryDeEnabledArray = Seq<(uint32, uint32)>;

pub struct S {
    pub telemetry_de_enabled_list_count: uint32,
    pub telemetry_de_enabled_list_index: uint32,
}

pub const TELEMETRY_DE_SUCCESS: int32 = 0;
pub const TELEMETRY_DE_ERR_INVALID: int32 = -1;
pub const TELEMETRY_DE_ERR_RANGE: int32 = -2;

pub const TELEMETRY_DE_COUNT_MASK: uint32 = 0xFFFF;
pub const TELEMETRY_DE_FLAG_SHIFT: uint32 = 16;
pub const TELEMETRY_DE_RESERVED_MASK: uint32 = 0xFFFFFFFC;
pub const TELEMETRY_DE_STATE_MASK: uint32 = 0x3;

} // verus!
