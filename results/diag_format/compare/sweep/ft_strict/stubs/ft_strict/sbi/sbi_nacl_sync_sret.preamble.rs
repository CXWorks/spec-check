use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub open spec fn NaclSharedMemoryCsrsSynchronized(s: S) -> bool;

pub open spec fn NaclSharedMemoryHfencesSynchronized(s: S) -> bool;

pub open spec fn SretEmulated(s: S) -> bool;

pub open spec fn ReturnsToCaller(s: S) -> bool;

} // verus!
