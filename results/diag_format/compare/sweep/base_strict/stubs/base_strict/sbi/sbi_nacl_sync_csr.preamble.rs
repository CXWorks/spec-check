use vstd::prelude::*;

verus! {

pub struct S {
    pub hcsrs: Seq<u64>,
    pub synchronized: Seq<bool>,
}

pub spec const csr_num: u64 = 0x600u64;

pub open spec fn AllOnes() -> u64;

pub open spec fn AllImplementedHCsrsSynchronized(s: S) -> bool;

pub open spec fn HCsrSynchronized(csr: u64, s: S) -> bool;

} // verus!
