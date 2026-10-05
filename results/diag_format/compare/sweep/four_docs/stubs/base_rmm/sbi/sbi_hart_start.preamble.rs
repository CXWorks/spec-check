use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub spec const hartid: u64 = 1;
pub spec const start_addr: u64 = 2;
pub spec const opaque: u64 = 3;

pub uninterp spec fn HartStartRequested(s: S, h: u64, sa: u64, op: u64) -> bool;

} // verus!
