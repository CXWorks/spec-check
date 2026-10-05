use vstd::prelude::*;
verus! {

pub struct S {
    pub num_voltage_domains: nat,
}

pub open spec fn NumVoltageDomains(s: S) -> int;

} // verus!
