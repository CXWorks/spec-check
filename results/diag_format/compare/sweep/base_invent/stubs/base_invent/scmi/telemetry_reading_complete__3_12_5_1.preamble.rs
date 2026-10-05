use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub type uint32 = u32;

pub struct S {
    pub telemetry_buffer: Seq<uint32>,
    pub telemetry_num_dwords: uint32,
    pub hardware_ok: bool,
}

pub const HARDWARE_ERROR: int32 = -1;

pub const PARTIAL_ERROR: int32 = -2;

pub const INTERFACE_ERROR: int32 = -3;

pub const SUCCESS: int32 = 0;

} // verus!
