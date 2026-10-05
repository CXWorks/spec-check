use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub num_voltage_domains: UInt32,
}

pub open spec fn NumVoltageDomains() -> UInt32;

} // verus!
