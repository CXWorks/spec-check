use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct SensorDescriptor {
    pub sensor_type: u32,
    pub flags: u32,
}

pub struct S {
    pub num_sensors: u32,
    pub initialized: bool,
}

pub const RSI_SUCCESS: int32 = 0;
pub const RSI_ERROR_INPUT: int32 = 1;
pub const RSI_ERROR_STATE: int32 = 2;

} // verus!
