use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub spec const channel_id: u64 = 0;

pub spec const base_attribute_id: u64 = 1;

pub spec const attribute_count: u64 = 2;

pub uninterp spec fn AttributesRead(ch_id: u64, base_attr_id: u64, attr_count: u64) -> bool;

} // verus!
