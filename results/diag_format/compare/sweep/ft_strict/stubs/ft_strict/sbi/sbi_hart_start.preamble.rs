use vstd::prelude::*;
verus! {

pub type unsigned_long = u64;

pub type long = i64;

pub struct sbiret {
    pub code: long,
    pub value: long,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn HartStartRequestedInSupervisorMode(s: S, hartid: unsigned_long, start_addr: unsigned_long) -> bool;

} // verus!
