use vstd::prelude::*;

verus! {

pub type long = i64;

pub type ulong = u64;

pub struct S {
    pub csr_num: ulong,
}

pub open spec fn AllImplementedHCsrsSynchronized(s: S) -> bool;

pub open spec fn HCsrSynchronized(s: S, csr_num: ulong) -> bool;

} // verus!
