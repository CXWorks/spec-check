use vstd::prelude::*;
verus! {

pub struct S {
    pub sensor_descriptor_count: int,
    pub returned_descriptors: Seq<int>,
}

pub open spec fn SensorDescriptorCount(s: S) -> int;

pub open spec fn ReturnedSensorDescriptorsMatch(old_s: S, new_s: S, start: int, count: int) -> bool;

} // verus!
