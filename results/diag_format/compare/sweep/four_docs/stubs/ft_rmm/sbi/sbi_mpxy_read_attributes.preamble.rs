use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type UInt64 = u64;

pub struct SbiRet {
    pub code: i64,
    pub union: (),
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn AttributesRead(s: S, channel_id: UInt32, base_attribute_id: UInt32, attribute_count: UInt32) -> bool;

} // verus!
