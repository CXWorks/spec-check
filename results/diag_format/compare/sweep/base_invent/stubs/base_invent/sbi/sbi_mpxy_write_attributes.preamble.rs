use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub mpxy_write_attributes_channel_id: u32,
    pub mpxy_write_attributes_base_attribute_id: u32,
    pub mpxy_write_attributes_attribute_count: u32,
}

} // verus!
