use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct PowercapMeasurementsGetState {
    pub power: uint32,
    pub mai: uint32,
}

pub struct S {
    pub powercap_measurements_get: PowercapMeasurementsGetState,
}

} // verus!
