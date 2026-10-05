use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub open spec fn NaclSharedMemCsrsSynchronized(s: S) -> bool;

pub open spec fn NaclSharedMemHfencesSynchronized(s: S) -> bool;

pub open spec fn SretEmulated(s: S) -> bool;

} // verus!
