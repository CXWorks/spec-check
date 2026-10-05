use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt16 = u16;
pub type UInt12 = u16;
pub type UInt64 = u64;

pub type RsiCommandReturnCode = u32;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;

pub spec const i: int = 0;

pub struct BitField64 {
    pub bits: u64,
}

impl BitField64 {
    pub open spec fn spec_index(self, idx: int) -> int;
}

pub struct SENSOR_DESC {
    pub sensor_id: UInt64,
    pub sensor_attributes_high: BitField64,
    pub sensor_power: UInt64,
}

pub struct S {
    pub sensor_flags: UInt64,
    pub sensor_descriptors: Seq<SENSOR_DESC>,
}

pub open spec fn SensorDescriptorArray(s: S, desc_index: UInt32) -> SENSOR_DESC;

pub open spec fn num_sensor_flags(s: S, hi: int, lo: int) -> int;

pub open spec fn NumRemainingSensorDescriptors(s: S, desc_index: UInt32, n: int) -> int;

pub open spec fn PreviousSensorDescription(s: S, sensor_id: UInt64) -> SENSOR_DESC;

} // verus!
