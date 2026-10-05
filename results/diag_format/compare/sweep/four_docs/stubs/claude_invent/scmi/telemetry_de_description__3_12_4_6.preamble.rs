use vstd::prelude::*;
verus! {

pub struct DeDesc {
    pub de_id: u32,
    pub de_attributes_1: u32,
    pub de_attributes_2: u32,
    pub de_attributes_3: u32,
}

pub struct S {
    pub de_descriptors: Seq<DeDesc>,
    pub version: u32,
}

pub open spec fn DeDescriptorArray(s: S) -> Seq<DeDesc>;

} // verus!
