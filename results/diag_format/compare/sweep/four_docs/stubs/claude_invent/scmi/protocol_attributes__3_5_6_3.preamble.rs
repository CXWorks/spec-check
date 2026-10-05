use vstd::prelude::*;
verus! {

pub struct S {
    pub attributes: u32,
    pub statistics_address_low: u32,
    pub statistics_address_high: u32,
    pub statistics_len: u32,
}

} // verus!
