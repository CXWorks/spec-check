use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub struct S {
    pub control: u32,
    pub sampling_rate: u32,
    pub group_identifier: u32,
    pub telemetry_enabled: bool,
    pub num_enabled_des: nat,
    pub num_enabled_groups: nat,
}

pub const SUCCESS: int32 = 0;
pub const INVALID_PARAMETERS: int32 = 1;
pub const OUT_OF_RANGE: int32 = 2;

pub open spec fn control_bits_31_9(s: S) -> int;

pub open spec fn control_bits_4_1(s: S) -> int;

pub open spec fn control_bits_8_5(s: S) -> int;

pub open spec fn control_bit_0(s: S) -> int;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

} // verus!
